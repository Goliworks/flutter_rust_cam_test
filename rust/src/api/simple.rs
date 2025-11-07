#[derive(Debug)]
pub struct Cameras {
    pub id: String,
    pub name: String,
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_image(file: String) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file)
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
            for camera in cameras {
                cams.push(Cameras {
                    id: camera.misc(),
                    name: camera.human_name(),
                });
            }
        }
        Err(e) => println!("Error: {e}"),
    }
    cams
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
