use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
};

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

pub fn stream_camera(id: u32) {
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
}
