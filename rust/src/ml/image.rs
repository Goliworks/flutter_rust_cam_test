use burn::tensor::Tensor;
use burn_ndarray::{NdArray, NdArrayDevice};

use super::my_model::Model;

const MASK_OFFSET_X: i32 = 28;
const MASK_OFFSET_Y: i32 = 18;
const MODEL_SIZE: u32 = 256;

pub struct ImageSegmentation {
    device: NdArrayDevice,
    model: Model<NdArray<f32>>,
}

impl ImageSegmentation {
    pub fn init() -> ImageSegmentation {
        ImageSegmentation {
            device: NdArrayDevice::default(),
            model: Model::default(),
        }
    }
    pub fn create_mask(&self, rgba_data: Vec<u8>) -> Vec<f32> {
        use image::DynamicImage;

        let rgba_img = image::RgbaImage::from_raw(640, 480, rgba_data).unwrap();
        let rgb_img = DynamicImage::ImageRgba8(rgba_img).to_rgb8();

        let resized_rgb: image::RgbImage = image::imageops::resize(
            &rgb_img,
            MODEL_SIZE,
            MODEL_SIZE,
            image::imageops::FilterType::Triangle,
        );

        let rgb_256 = resized_rgb.into_raw();

        let normalized: Vec<f32> = rgb_256.iter().map(|&p| p as f32 / 255.0).collect();

        // Start burn inference.
        let input = Tensor::<NdArray, 1>::from_floats(normalized.as_slice(), &self.device)
            .reshape([1, 256, 256, 3])
            .swap_dims(1, 3) // [1, 3, 256, 256]
            .swap_dims(2, 3);

        let output = self.model.forward(input);
        let mask_data = output.into_data().to_vec::<f32>().unwrap();

        // Create a mask image and resize it to 640x480.
        let mask_img = image::ImageBuffer::from_fn(MODEL_SIZE, MODEL_SIZE, |x, y| {
            let idx = (y * MODEL_SIZE + x) as usize;
            let val = (mask_data[idx] * 255.0) as u8;
            image::Luma([val])
        });

        let resized_mask =
            image::imageops::resize(&mask_img, 640, 480, image::imageops::FilterType::Triangle);

        // Convert mask to Vec<f32>
        let mask: Vec<f32> = resized_mask.pixels().map(|p| p[0] as f32 / 255.0).collect();

        mask
    }
}

pub fn blur_background(rgba_data: &[u8], mask: &[f32], blur_sigma: f32) -> Vec<u8> {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    let original = image::RgbaImage::from_raw(WIDTH, HEIGHT, rgba_data.to_vec())
        .expect("Invalid image dimensions");
    let blurred = image::imageops::blur(&original, blur_sigma);

    blend_images(rgba_data, blurred.as_raw(), mask)
}

// note : Duplication from blur_background. to improve.
pub fn replace_background(rgba_data: &[u8], background_rgba_data: &[u8], mask: &[f32]) -> Vec<u8> {
    blend_images(rgba_data, background_rgba_data, mask)
}

fn blend_images(rgba_data: &[u8], background_rgba_data: &[u8], mask: &[f32]) -> Vec<u8> {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    const TOTAL_PIXELS: usize = WIDTH * HEIGHT;

    let mut result = vec![0u8; TOTAL_PIXELS * 4];

    for i in 0..TOTAL_PIXELS {
        let x = (i % WIDTH) as i32;
        let y = (i / WIDTH) as i32;

        // Calculer l'index du masque avec offset
        let mask_x = (x + MASK_OFFSET_X).clamp(0, WIDTH as i32 - 1) as usize;
        let mask_y = (y + MASK_OFFSET_Y).clamp(0, HEIGHT as i32 - 1) as usize;
        let mask_idx = mask_y * WIDTH + mask_x;
        let alpha = mask[mask_idx];
        let inv_alpha = 1.0 - alpha;

        let pixel_idx = i * 4;

        // Blending RGB (sans conversion via get_pixel)
        result[pixel_idx] = ((rgba_data[pixel_idx] as f32 * alpha)
            + (background_rgba_data[pixel_idx] as f32 * inv_alpha))
            as u8;
        result[pixel_idx + 1] = ((rgba_data[pixel_idx + 1] as f32 * alpha)
            + (background_rgba_data[pixel_idx + 1] as f32 * inv_alpha))
            as u8;
        result[pixel_idx + 2] = ((rgba_data[pixel_idx + 2] as f32 * alpha)
            + (background_rgba_data[pixel_idx + 2] as f32 * inv_alpha))
            as u8;
        result[pixel_idx + 3] = rgba_data[pixel_idx + 3]; // Alpha channel
    }

    result
}

// Used for debug.
// apply green overlay on the mask.
pub fn show_mask_overlay(rgba_data: &[u8], mask: &[f32]) -> Vec<u8> {
    let mut result = rgba_data.to_vec();

    const WIDTH: i32 = 640;
    const HEIGHT: i32 = 480;

    for i in 0..(WIDTH * HEIGHT) as usize {
        let x = (i as i32) % WIDTH;
        let y = (i as i32) / WIDTH;

        let mask_x = (x + MASK_OFFSET_X).clamp(0, WIDTH - 1);
        let mask_y = (y + MASK_OFFSET_Y).clamp(0, HEIGHT - 1);
        let mask_idx = (mask_y * WIDTH + mask_x) as usize;
        let mask_val = mask[mask_idx];

        if mask_val > 0.5 {
            let idx = i * 4;
            result[idx] = (result[idx] as f32 * 0.5) as u8;
            result[idx + 1] = ((result[idx + 1] as f32 * 0.5) + 127.0) as u8;
            result[idx + 2] = (result[idx + 2] as f32 * 0.5) as u8;
        }
    }

    result
}
