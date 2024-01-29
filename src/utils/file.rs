use std::io::BufWriter;
use std::num::NonZeroU32;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use fast_image_resize as fir;

use crate::error::CustomError;

pub async fn resize_png_from_path(path: &str) -> Result<(), CustomError> {
    let img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fir::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fir::PixelType::U8x4,
    ).unwrap();

    let alpha_mul_div = fir::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    let dst_width = NonZeroU32::new(800).unwrap();
    let dst_height = NonZeroU32::new(600).unwrap();
    let mut dst_image = fir::Image::new(
        dst_width,
        dst_height,
        src_image.pixel_type(),
    );

    let mut dst_view = dst_image.view_mut();

    let mut resizer = fir::Resizer::new(
        fir::ResizeAlg::Convolution(fir::FilterType::Lanczos3),
    );
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    let mut result_buf = BufWriter::new(Vec::new());
    let encoder = PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        );
    match encoder {
        Ok(()) => Ok(()),
        Err(_err) => Err(CustomError::ResizeImageError)
    }
}
