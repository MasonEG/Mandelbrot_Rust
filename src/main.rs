/*
    Mostly made by me, used https://github.com/image-rs/image/blob/master/examples/fractal.rs
    as a reference

    process is fairly standard, I learned it from a bunch of websites
*/


extern crate image;
extern crate num_complex;

use num_complex::Complex;

fn main() {
    let resx = 3000;
    let resy = 3000;

    let mut img_buf = image::ImageBuffer::new(resx, resy);

    for (x, y , pixel) in img_buf.enumerate_pixels_mut() {

        let col: u8 = 1;
        *pixel = image::Rgb([col, col, col]);

        let a = x as f32 * (2.5 / resx as f32) - 2.0;
        let b = y as f32 *  (2.5 / resy as f32) - 1.25;

        let c = Complex::new(a, b);
        let mut z = Complex::new(0 as f32, 0 as f32);

        for _i in 0..99 {
            z = z * z + c;
            let norm = z.norm();
            if norm > 2.0 {
                let mut r: u8 = 0;
                let mut g: u8 = 0;
                let mut b: u8 = 0;

                let range_val = if norm <= 4.0 { norm - 2.0 } else { 2.0 };

                let r_factor = if range_val <= 1.5 { (range_val - 0.5).abs() }
                    else { (range_val - 2.5).abs() };
                let g_factor = (range_val - 1.0).abs();
                let b_factor = if range_val >= 0.5 { (range_val - 1.5).abs() }
                    else { (range_val + 0.5).abs() };

                // println!("rfac: {}, gfac: {}, bfac: {}", r_factor, g_factor, b_factor);

                // if r_factor <= 1.0 {
                //     r = (100.0 * r_factor) as u8;
                // } else { println!("r check"); }
                // if g_factor <= 1.0 {
                //     g = (100.0 * g_factor) as u8;
                // } else { println!("g check"); }
                // if b_factor <= 1.0 {
                //     b = (100.0 * b_factor) as u8;
                // } else { println!("b check"); }

                // if norm < 2.5 {
                //     r = 100;
                // } else if norm < 3.0 {
                //     g = 100;
                // } else {
                //     b = 100;
                // }

                //keep me from making dumb calculations
                if b > 100 || g > 100 || r > 100 {
                    println!("ERROR! rgb values are wrong! r: {}, g: {}, b: {}", r, g, b);
                }

                // println!("r: {}, g: {}, b: {}", r, g, b);

                *pixel = image::Rgb([r, g, b]);
                break;
            }
        }
    }

    img_buf.save("mandelbrot.png").unwrap();
}
