#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_image(file: String) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file)
}

pub fn init_cam() {
    nokhwa::nokhwa_initialize(|x| {
        println!("nokhwa initialized: {x}");
    });
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
