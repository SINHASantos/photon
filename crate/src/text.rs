//! Draw text onto an image.
//! For extended graphic design/text-drawing functionality, see [GDL](https://github.com/silvia-odwyer/gdl),
//! which is a graphic design library, compatible with Photon.

use crate::iter::ImageIterator;
use crate::{helpers, PhotonImage};
use image::{DynamicImage, Rgba};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use rusttype::{Font, Scale};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Add bordered-text to an image.
/// The only font available as of now is Roboto.
/// Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
///
/// # Example
///
/// ```no_run
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_border;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let mut image2: DynamicImage =
        DynamicImage::new_luma8(image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_bytes(&font).unwrap();
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };
    // Draw the border using a grayscale image; we use white by default.
    draw_text_mut(
        &mut image2,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );

    let mut image2 = image2.to_luma8();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    // Add a border to the text.
    for (x, y) in ImageIterator::with_dimension(&image2.dimensions()) {
        let pixval = 255 - image2.get_pixel(x, y)[0];
        if pixval != 255 {
            let new_pix = Rgba([pixval, pixval, pixval, 255]);
            image.put_pixel(x, y, new_pix);
        }
    }

    // Draw the text itself in white by default.
    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image.
/// The only font available as of now is Roboto.
/// Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
///
/// # Example
///
/// ```no_run
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_bytes(&font).unwrap();
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image with a custom RGBA colour.
///
/// This function allows you to specify the colour of the text via individual
/// red, green, blue and alpha channel values. It follows the same semantics
/// as [`draw_text`], but uses the supplied colour when drawing the text.
///
/// # Arguments
/// * `photon_img` - A mutable reference to the PhotonImage to draw on.
/// * `text` - The string slice containing the text you want to draw.
/// * `x` - The x-coordinate for the first letter's top-left pixel.
/// * `y` - The y-coordinate for the first letter's top-left pixel.
/// * `font_size` - The font size in pixels.
/// * `r` - The red channel of the text colour (0‒255).
/// * `g` - The green channel of the text colour (0‒255).
/// * `b` - The blue channel of the text colour (0‒255).
/// * `a` - The alpha (opacity) channel of the text colour (0‒255).
///
/// # Example
///
/// ```no_run
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_color;
/// let mut img = open_image("img.jpg").expect("File should open");
/// // Draw semi-transparent red text at position (50, 50)
/// draw_text_with_color(&mut img, "Hello, world!", 50, 50, 42.0, 255, 0, 0, 128);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_color(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_bytes(&font).unwrap();
    let scale = Scale {
        x: font_size * 1.0,
        y: font_size,
    };

    draw_text_mut(
        &mut image,
        Rgba([r, g, b, a]),
        x,
        y,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}
