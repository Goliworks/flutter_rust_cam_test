use std::{
    sync::{atomic::AtomicBool, Arc, Mutex, OnceLock},
    thread,
};

use image::ImageReader;
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Buffer,
};

use crate::ml::image::{blur_background, show_mask_overlay, ImageSegmentation};
use crate::{frb_generated::StreamSink, ml::image::replace_background};

#[derive(Debug)]
pub struct Cameras {
    pub id: String,
    pub name: String,
}

pub fn init_cams() {
    nokhwa::nokhwa_initialize(|x| {
        println!("nokhwa initialized: {x}");
    });
}

pub fn check_for_cameras() -> Vec<Cameras> {
    let mut cams: Vec<Cameras> = Vec::new();
    match nokhwa::query(nokhwa::native_api_backend().unwrap()) {
        Ok(cameras) => {
            println!("Cameras: {cameras:?}");
            for (i, camera) in cameras.iter().enumerate() {
                cams.push(Cameras {
                    id: i.to_string(),
                    name: camera.human_name(),
                });
            }
        }
        Err(e) => println!("Error: {e}"),
    }
    cams
}

struct CameraState {
    mask: Arc<AtomicBool>,
    background: Arc<Mutex<Option<Vec<u8>>>>,
    debug: Arc<AtomicBool>,
}

static CAMERA_STATE: OnceLock<Arc<CameraState>> = OnceLock::new();

pub fn set_mask(mask: bool) {
    let state = CAMERA_STATE.get_or_init(|| {
        Arc::new(CameraState {
            mask: Arc::new(AtomicBool::new(mask)),
            background: Arc::new(Mutex::new(None)),
            debug: Arc::new(AtomicBool::new(false)),
        })
    });
    state.mask.store(mask, std::sync::atomic::Ordering::Relaxed);
    state.background.lock().unwrap().take();
}

pub fn set_background(background: Vec<u8>) {
    let img = ImageReader::new(std::io::Cursor::new(&background))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let resized = img.resize_exact(640, 480, image::imageops::FilterType::Lanczos3);
    let buf = resized.to_rgba8().into_raw();

    let state = CAMERA_STATE.get_or_init(|| {
        Arc::new(CameraState {
            mask: Arc::new(AtomicBool::new(false)),
            background: Arc::new(Mutex::new(Some(buf.clone()))),
            debug: Arc::new(AtomicBool::new(false)),
        })
    });
    state
        .mask
        .store(false, std::sync::atomic::Ordering::Relaxed);
    state.background.lock().unwrap().replace(buf);
}

pub fn set_debug(debug: bool) {
    let state = CAMERA_STATE.get_or_init(|| {
        Arc::new(CameraState {
            mask: Arc::new(AtomicBool::new(false)),
            background: Arc::new(Mutex::new(None)),
            debug: Arc::new(AtomicBool::new(debug)),
        })
    });
    state
        .debug
        .store(debug, std::sync::atomic::Ordering::Relaxed);
}

pub fn stream_camera(id: u32, sink: StreamSink<Vec<u8>>) -> Result<(), std::io::Error> {
    let latest_frame = Arc::new(Mutex::new(None::<Buffer>));
    let frame_for_capture = latest_frame.clone();

    let should_run = Arc::new(AtomicBool::new(true));
    let should_run_capture = should_run.clone();

    thread::spawn(move || {
        let requested = RequestedFormat::new::<RgbAFormat>(RequestedFormatType::Closest(
            nokhwa::utils::CameraFormat::new(
                nokhwa::utils::Resolution::new(640, 480),
                nokhwa::utils::FrameFormat::YUYV,
                30,
            ),
        ));

        let mut camera = nokhwa::Camera::new(nokhwa::utils::CameraIndex::Index(id), requested)
            .expect("Can't access camera");
        camera.open_stream().expect("Can't start camera stream");

        while should_run_capture.load(std::sync::atomic::Ordering::Relaxed) {
            match camera.frame() {
                Ok(frame) => {
                    let mut slot = frame_for_capture.lock().unwrap();
                    *slot = Some(frame);
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    thread::sleep(std::time::Duration::from_millis(33));
                }
            }
        }
        let _ = camera.stop_stream();
        println!("Camera thread stopped");
    });

    let frame_for_processing = latest_frame.clone();

    thread::spawn(move || {
        let mut buffer = vec![0u8; 640 * 480 * 4];
        let is = ImageSegmentation::init();

        while should_run.load(std::sync::atomic::Ordering::Relaxed) {
            let frame_opt = {
                let mut slot = frame_for_processing.lock().unwrap();
                slot.take()
            };

            let Some(frame) = frame_opt else {
                thread::sleep(std::time::Duration::from_millis(5));
                continue;
            };

            frame
                .decode_image_to_buffer::<RgbAFormat>(&mut buffer)
                .unwrap();

            let debug = CAMERA_STATE
                .get()
                .unwrap()
                .debug
                .load(std::sync::atomic::Ordering::Relaxed);

            let has_mask = CAMERA_STATE
                .get()
                .unwrap()
                .mask
                .load(std::sync::atomic::Ordering::Relaxed);

            let background = CAMERA_STATE
                .get()
                .unwrap()
                .background
                .lock()
                .unwrap()
                .clone();

            let mut final_image: Vec<u8>;

            if !debug {
                if has_mask {
                    let mask = is.create_mask(buffer.clone());
                    final_image = blur_background(&buffer, &mask, 12.0);
                } else if let Some(background) = background {
                    let mask = is.create_mask(buffer.clone());
                    final_image = replace_background(&buffer, &background, &mask);
                } else {
                    final_image = buffer.clone();
                }
            } else {
                let mask = is.create_mask(buffer.clone());
                final_image = show_mask_overlay(&buffer, &mask);
            }

            // Stop the loop if flutter close the stream.
            if sink.add(final_image).is_err() {
                should_run.store(false, std::sync::atomic::Ordering::Relaxed);
                break;
            };
        }
        println!("Processing thread stopped");
    });

    Ok(())
}
