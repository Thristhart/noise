extern crate image;
extern crate rand;

type FloatGrayImage = image::ImageBuffer<image::Luma<f32>, Vec<f32>>;

// we have to work in floats for the algorithm to work nicely (negative values are important)
// but the resulting image should be u8s
fn convert_to_gray_image(img: FloatGrayImage) -> image::GrayImage {
    let mut converted_img = image::GrayImage::new(img.width(), img.height());
    for (x, y, pixel) in img.enumerate_pixels() {
        converted_img.put_pixel(x, y, image::Luma([(pixel.data[0] * 255.0).round() as u8]));
    }
    return converted_img;
}

pub fn white_noise_float(width: u32, height: u32) -> FloatGrayImage {
    let mut imgbuf = FloatGrayImage::new(width, height);

    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Luma([rand::random()]);
    }

    return imgbuf;
}

pub fn white_noise(width: u32, height: u32) -> image::GrayImage {
    return convert_to_gray_image(white_noise_float(width, height));
}

pub fn red_noise(width: u32, height: u32, iterations: u8, sigma: f32) -> image::GrayImage {
    let mut noise = white_noise_float(width, height);

    for _ in 0..iterations {
        noise = image::imageops::blur(&noise, sigma);
        normalize_histogram(&mut noise);
    }

    return convert_to_gray_image(noise);
}

pub fn blue_noise(width: u32, height: u32, iterations: u8, sigma: f32) -> image::GrayImage {
    let mut noise = white_noise_float(width, height);

    for _ in 0..iterations {
        let blurred = image::imageops::blur(&noise, sigma);

        for (x, y, blurred_pixel) in blurred.enumerate_pixels() {
            let base_pixel = noise.get_pixel_mut(x, y);
            base_pixel.data[0] = base_pixel.data[0] - blurred_pixel.data[0];
        }
        
        normalize_histogram(&mut noise);
    }

    return convert_to_gray_image(noise);
}

pub fn green_noise(width: u32, height: u32, iterations: u8, low_sigma: f32, high_sigma: f32) -> image::GrayImage {
    let mut noise = white_noise_float(width, height);

    for _ in 0..iterations {
        let mut low_blurred = image::imageops::blur(&noise, low_sigma);
        let high_blurred = image::imageops::blur(&noise, high_sigma);

        for (x, y, blurred_pixel) in high_blurred.enumerate_pixels() {
            let base_pixel = low_blurred.get_pixel_mut(x, y);
            base_pixel.data[0] = base_pixel.data[0] - blurred_pixel.data[0];
        }
        
        normalize_histogram(&mut low_blurred);

        noise = low_blurred;
    }

    return convert_to_gray_image(noise);
}

pub fn purple_noise(width: u32, height: u32, iterations: u8, low_sigma: f32, high_sigma: f32) -> image::GrayImage {
    let mut noise = white_noise_float(width, height);

    for _ in 0..iterations {
        let mut low_blurred = image::imageops::blur(&noise, low_sigma);
        let high_blurred = image::imageops::blur(&noise, high_sigma);

        for (x, y, blurred_pixel) in high_blurred.enumerate_pixels() {
            let base_pixel = low_blurred.get_pixel_mut(x, y);
            base_pixel.data[0] = base_pixel.data[0] - blurred_pixel.data[0];
        }

        // low_blurred is now the middle frequency data
        
        for (x, y, mid_freq_pixel) in low_blurred.enumerate_pixels() {
            let base_pixel = noise.get_pixel_mut(x, y);
            base_pixel.data[0] = base_pixel.data[0] - mid_freq_pixel.data[0];
        }

        normalize_histogram(&mut noise);
    }

    return convert_to_gray_image(noise);
}

fn normalize_histogram(img: &mut FloatGrayImage) {
    let pixel_count = img.width() * img.height();

    let mut pixels: Vec<&mut image::Luma<f32>> = Vec::with_capacity(pixel_count as usize);

    pixels.extend(img.pixels_mut());
    pixels.sort_unstable_by(|a, b| b.data[0].partial_cmp(&a.data[0]).unwrap_or(std::cmp::Ordering::Equal));

    for (index, pixel) in pixels.iter_mut().enumerate() {
        let value: f32 = index as f32 / pixel_count as f32;
        pixel.data[0] = value;
    }
}