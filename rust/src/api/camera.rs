use std::thread;

use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
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

pub fn stream_camera(id: u32, sink: StreamSink<Vec<u8>>) -> Result<(), std::io::Error> {
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

        let mut buffer = vec![0u8; 640 * 480 * 4];
        let is = ImageSegmentation::init();
        loop {
            match camera.frame() {
                Ok(frame) => {
                    println!("Frame");
                    frame
                        .decode_image_to_buffer::<RgbAFormat>(&mut buffer)
                        .unwrap();

                    let mask = is.create_mask(buffer.clone());

                    let final_image = blur_background(&buffer, &mask, 10.0);
                    // let final_image = show_mask_overlay(&buffer, &mask);

                    // Stop the loop if flutter close the stream.
                    if sink.add(final_image).is_err() {
                        break;
                    };
                }
                Err(e) => {
                    println!("Error: {e}");
                    thread::sleep(std::time::Duration::from_millis(33));
                }
            }
        }

        let _ = camera.stop_stream();
        println!("Camera thread stopped");
    });

    Ok(())
}
