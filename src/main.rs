use image::{ImageBuffer, RgbImage};
use opencv::{
    Result,
    core::{self, Scalar},
    imgproc,
    prelude::*,
};
fn main() -> Result<()> {
    let width = 400;
    let height = 400;
    let mut mat = Mat::new_rows_cols_with_default(height, width, core::CV_8UC3, Scalar::all(0.0))?;

    imgproc::rectangle(
        &mut mat,
        core::Rect::new(50, 50, 100, 100),
        Scalar::new(0.0, 0.0, 255.0, 0.0), // BGR
        2,
        imgproc::LINE_8,
        0,
    )?;

    imgproc::circle(
        &mut mat,
        core::Point::new(200, 200),
        50,
        Scalar::new(0.0, 255.0, 0.0, 0.0), // BGR
        -1,
        imgproc::LINE_AA,
        0,
    )?;

    imgproc::put_text(
        &mut mat,
        "Hello imgproc",
        core::Point::new(70, 300),
        imgproc::FONT_HERSHEY_SIMPLEX,
        1.0,
        Scalar::new(255.0, 0.0, 0.0, 0.0), // BGR
        2,
        imgproc::LINE_AA,
        false,
    )?;

    let mut rgb = Mat::default();
    imgproc::cvt_color(
        &mat,
        &mut rgb,
        imgproc::COLOR_BGR2RGB,
        0,
        core::AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    let rgb_data = rgb.data_bytes()?;

    let image: RgbImage = ImageBuffer::from_raw(width as u32, height as u32, rgb_data.to_vec())
        .expect("Failed to create image");

    image.save("output.png").expect("Failed to save image");

    println!("Saved output.png using image crate");

    Ok(())
}
