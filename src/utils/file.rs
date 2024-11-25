use std::io::BufWriter;

use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use fast_image_resize as fir;

use crate::error::CustomError;

pub async fn resize_png_from_path(
    width: u32,
    height: u32,
    buffer: Vec<u8>,
    pixel_type: fir::PixelType,
) -> Result<(), CustomError> {
    let src_image = fir::images::Image::from_vec_u8(width, height, buffer, pixel_type).unwrap();

    // Create container for data of destination image
    let dst_width = 1024;
    let dst_height = 768;
    let mut dst_image = fir::images::Image::new(dst_width, dst_height, fir::PixelType::U8);

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fir::Resizer::new();
    resizer.resize(&src_image, &mut dst_image, None).unwrap();

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    let new_image = PngEncoder::new(&mut result_buf).write_image(
        dst_image.buffer(),
        dst_width,
        dst_height,
        image::ExtendedColorType::Rgba8,
    );
    match new_image {
        Ok(()) => Ok(()),
        Err(_err) => Err(CustomError::ResizeImageError),
    }
}
