pub mod image;
pub mod my_model {
    include!(concat!(
        env!("OUT_DIR"),
        "/model/selfie_segmentation_converted.rs"
    ));
}
