use std::os::raw::c_int;

mod cs50;

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct RGBTRIPLE {
    pub rgbtBlue: u8,
    pub rgbtGreen: u8,
    pub rgbtRed: u8,
}

#[no_mangle]
pub unsafe extern "C" fn grayscale_rs(height: c_int, width: c_int, image: *mut RGBTRIPLE) {
    let mut image_matrix = cs50::Array2D::new(
        cs50::c_arr_to_slice(image, height * width),
        width as usize,
        height as usize,
    );
    grayscale(&mut image_matrix);
}

fn grayscale(image: &mut cs50::Array2D<'_, RGBTRIPLE>) {
    for row in 0..image.height {
        for col in 0..image.width {
            let pixel = *image.get(row, col);

            let avg   = ((
                pixel.rgbtRed as u16 +
                pixel.rgbtGreen as u16 +
                pixel.rgbtBlue as u16
            ) / 3) as u8;

            let out = image.get_mut(row, col);
            out.rgbtRed   = avg;
            out.rgbtGreen = avg;
            out.rgbtBlue  = avg;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sepia_rs(height: c_int, width: c_int, image: *mut RGBTRIPLE) {
    let mut image_matrix = cs50::Array2D::new(
        cs50::c_arr_to_slice(image, height * width),
        width as usize,
        height as usize,
    );
    sepia(&mut image_matrix);
}

fn sepia(image: &mut cs50::Array2D<'_, RGBTRIPLE>) {
    for row in 0..image.height {
        for col in 0..image.width {
            let pixel = *image.get(row, col);

            let sepia_convert = |cr: f32, cg: f32, cb: f32| -> u8 {
                (cr * pixel.rgbtRed as f32 + 
                 cg * pixel.rgbtGreen as f32 + 
                 cb * pixel.rgbtBlue as f32)
                .max(0.0).min(255.0).round() as u8
            };

            let out = image.get_mut(row, col);
            out.rgbtRed   = sepia_convert(0.393, 0.769, 0.189);
            out.rgbtGreen = sepia_convert(0.349, 0.686, 0.168);
            out.rgbtBlue  = sepia_convert(0.272, 0.534, 0.131);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn reflect_rs(height: c_int, width: c_int, image: *mut RGBTRIPLE) {
    let mut image_matrix = cs50::Array2D::new(
        cs50::c_arr_to_slice(image, height * width),
        width as usize,
        height as usize,
    );
    reflect(&mut image_matrix);
}

fn reflect(image: &mut cs50::Array2D<'_, RGBTRIPLE>) {
    for row in image.rows_mut() {
        let mut l: usize = 0;
        // saturating_sub(x) => row.len() - x but min is 0
        let mut r = row.len().saturating_sub(1); 
        while l < r {
            row.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn blur_rs(height: c_int, width: c_int, image: *mut RGBTRIPLE) {
    let mut image_matrix = cs50::Array2D::new(
        cs50::c_arr_to_slice(image, height * width),
        width as usize,
        height as usize,
    );
    blur(&mut image_matrix);
}

fn blur(image: &mut cs50::Array2D<'_, RGBTRIPLE>) {
    let original = image.to_owned();

    for row in 0..image.height {
        for col in 0..image.width {
            let mut sum_red: u16 = 0;
            let mut sum_green: u16 = 0;
            let mut sum_blue: u16 = 0;
        
            let mut count = 0;

            let row_start = row.saturating_sub(1);
            let row_end   = (row + 1).min(image.height - 1);
            let col_start = col.saturating_sub(1);
            let col_end   = (col + 1).min(image.width - 1);

            for i in row_start..=row_end {
                for j in col_start..=col_end {
                    let pixel = original.get(i, j);

                    sum_red   += pixel.rgbtRed as u16;
                    sum_green += pixel.rgbtGreen as u16;
                    sum_blue  += pixel.rgbtBlue as u16;

                    count += 1;
                }
            }

            let out = image.get_mut(row, col);
            out.rgbtRed   = (sum_red / count) as u8;
            out.rgbtGreen = (sum_green / count) as u8;
            out.rgbtBlue  = (sum_blue / count) as u8;
        }
    }
}