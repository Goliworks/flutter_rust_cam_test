use std::{
    sync::{atomic::AtomicBool, Arc, Mutex, OnceLock},
    thread,
};

use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Buffer,
};

use crate::frb_generated::StreamSink;
use crate::ml::image::{blur_background, show_mask_overlay, ImageSegmentation};

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
}

static CAMERA_STATE: OnceLock<Arc<CameraState>> = OnceLock::new();

pub fn set_mask(mask: bool) {
    let state = CAMERA_STATE.get_or_init(|| {
        Arc::new(CameraState {
            mask: Arc::new(AtomicBool::new(mask)),
        })
    });
    state.mask.store(mask, std::sync::atomic::Ordering::Relaxed);
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

            let has_mask = CAMERA_STATE
                .get()
                .unwrap()
                .mask
                .load(std::sync::atomic::Ordering::Relaxed);

            let mut final_image: Vec<u8>;
            if has_mask {
                let mask = is.create_mask(buffer.clone());
                final_image = blur_background(&buffer, &mask, 10.0);
            } else {
                final_image = buffer.clone();
            }
            // let final_image = show_mask_overlay(&buffer, &mask);

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
