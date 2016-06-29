extern crate image;

use image::GenericImage;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: img_diff [PNG file 1] [PNG file 2]");
        return;
    } else if args.len() != 3 {
        // Require 2 arguments
        panic!("Error! Usage: img_diff [PNG file 1] [PNG file 2]");
    }

    let file1 = args[1].to_string();
    let file2 = args[2].to_string();

    let img1 = image::open(file1).unwrap();
    let img2 = image::open(file2).unwrap();

    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();

    let widest = if width1 > width2 { width1 } else { width2 };
    let highest = if height1 > height2 { height1 } else { height2 };

    let mut diff = 0;

    for x in 0..widest {
        for y in 0..highest {
            if x < width1 && y < height1 && x < width2 && y < height2 {
                let pixel1 = img1.get_pixel(x, y);
                let pixel2 = img2.get_pixel(x, y);

                if pixel1[0] != pixel2[0] || pixel1[1] != pixel2[1] || pixel1[2] != pixel2[2] || pixel1[3] != pixel2[3] {
                    diff = diff + 1;
                }
            } else {
                diff = diff + 1;
            }
        }
    }

    let q = 100.0 * diff as f32 / (widest * highest) as f32;

    println!("{:.*}%", 0, q);
}
