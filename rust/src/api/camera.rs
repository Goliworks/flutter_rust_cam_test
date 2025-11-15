use std::thread;

use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
};

use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};
use my_model::Model;

use crate::{api::my_model, frb_generated::StreamSink};

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

const MODEL_SIZE: u32 = 256;

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
        let device = NdArrayDevice::default();
        let model: Model<NdArray<f32>> = Model::default();

        loop {
            match camera.frame() {
                Ok(frame) => {
                    println!("Frame");
                    frame
                        .decode_image_to_buffer::<RgbAFormat>(&mut buffer)
                        .unwrap();

                    let rgba_img = image::RgbaImage::from_raw(640, 480, buffer.clone()).unwrap();

                    let rgb_img = image::RgbImage::from_fn(640, 480, |x, y| {
                        let p = rgba_img.get_pixel(x, y);
                        image::Rgb([p[0], p[1], p[2]])
                    });

                    let resized_rgb: image::RgbImage = image::imageops::resize(
                        &rgb_img,
                        256,
                        256,
                        image::imageops::FilterType::Triangle,
                    );

                    let rgb_256 = resized_rgb.clone().into_raw();

                    let normalized: Vec<f32> = rgb_256.iter().map(|&p| p as f32 / 255.0).collect();

                    // Start burn inference.
                    let input = Tensor::<NdArray, 1>::from_floats(normalized.as_slice(), &device)
                        .reshape([1, 256, 256, 3])
                        .swap_dims(1, 3) // [1, 3, 256, 256]
                        .swap_dims(2, 3);

                    let output = model.forward(input);
                    let mask_data = output.into_data().to_vec::<f32>().unwrap();

                    // Create a mask image and resize it to 640x480.
                    let mask_img = image::ImageBuffer::from_fn(MODEL_SIZE, MODEL_SIZE, |x, y| {
                        let idx = (y * MODEL_SIZE + x) as usize;
                        let val = (mask_data[idx] * 255.0) as u8;
                        image::Luma([val])
                    });

                    let resized_mask = image::imageops::resize(
                        &mask_img,
                        640,
                        480,
                        image::imageops::FilterType::Triangle,
                    );

                    // Convert mask to Vec<f32>
                    let mask: Vec<f32> =
                        resized_mask.pixels().map(|p| p[0] as f32 / 255.0).collect();

                    let final_image = blur_background(&buffer, &mask, 10.0);

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

pub fn blur_background(rgba_data: &[u8], mask: &[f32], blur_sigma: f32) -> Vec<u8> {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let original = image::RgbaImage::from_raw(WIDTH, HEIGHT, rgba_data.to_vec())
        .expect("Invalid image dimensions");

    let blurred = image::imageops::blur(&original, blur_sigma);

    // blend the original image with the blurred image.
    let mut result = image::RgbaImage::new(WIDTH, HEIGHT);

    for (x, y, pixel) in result.enumerate_pixels_mut() {
        let idx = (y * WIDTH + x) as usize;
        let alpha = mask[idx];

        let orig = original.get_pixel(x, y);
        let blur = blurred.get_pixel(x, y);

        *pixel = image::Rgba([
            ((orig[0] as f32 * alpha) + (blur[0] as f32 * (1.0 - alpha))) as u8,
            ((orig[1] as f32 * alpha) + (blur[1] as f32 * (1.0 - alpha))) as u8,
            ((orig[2] as f32 * alpha) + (blur[2] as f32 * (1.0 - alpha))) as u8,
            orig[3], // keep original alpha
        ]);
    }

    result.into_raw()
}
