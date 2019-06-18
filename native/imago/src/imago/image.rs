use image::Rgba;

pub fn luminance(r: u8, g: u8, b: u8) -> u8 {
    ((r as f64 * 0.3) + (g as f64 * 0.59) + (b as f64 * 0.11)) as u8
}

pub fn pixel_luminance(pixel: (u32, u32, Rgba<u8>)) -> u8 {
    luminance(pixel.2.data[0], pixel.2.data[1], pixel.2.data[2])
}