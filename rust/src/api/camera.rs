use std::thread;

use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
};

use crate::frb_generated::StreamSink;

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

        loop {
            match camera.frame() {
                Ok(frame) => {
                    println!("Generate frame");
                    let img_data = frame.decode_image::<RgbAFormat>().unwrap().to_vec();
                    if sink.add(img_data).is_err() {
                        break;
                    };
                }
                Err(e) => println!("Error: {e}"),
            }
            thread::sleep(std::time::Duration::from_millis(33));
        }
    });

    Ok(())
}
