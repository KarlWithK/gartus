// extern crate rand;
use curves_rs::graphics::display::Canvas;
use curves_rs::graphics::display::Pixel;
// use curves_rs::utils;
// use rand::Rng;

#[test]
fn rgb() {
    let mut img = Canvas::empty(256, 256, 255);
    let (width, height) = (img.width(), img.height());
    let mut data: Vec<Pixel> = Vec::with_capacity((width * height) as usize);
    (0..height).rev().for_each(|j| {
        eprintln!("Scanlines reminaing: {}", height - j - 1);
        (0..width).for_each(|i| {
            data.push(Pixel {
                red: (255.99 * (i as f64 / (width - 1) as f64)) as u8,
                green: (255.99 * (j as f64 / (height - 1) as f64)) as u8,
                blue: (255.99 * 0.25) as u8,
            })
        });
    });
    eprintln!("Done.");
    img.fill_canvas(data);
    img.display().expect("Could not render image")
}

#[test]
fn julia() {
    let width = 800;
    let height = 600;
    let mut julia = Canvas::empty(height, width, 255);
    let mut data: Vec<Pixel> = Vec::with_capacity((width * height) as usize);
    let cx = -0.9;
    let cy = 0.27015;
    let interations = 110;
    for x in 0..width {
        for y in 0..height {
            let mut zx = 3.0 * (x as f32 - 0.5 * width as f32) / (width as f32);
            let mut zy = 2.0 * (y as f32 - 0.5 * height as f32) / (height as f32);
            let mut i = interations;
            while zx * zx + zy * zy < 4.0 && i > 1 {
                let temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                i -= 1;
            }
            data.push(Pixel {
                red: (i << 3) as u8,
                green: (i << 5) as u8,
                blue: (i << 4) as u8,
            })
            // write!(writer, "{} {} {} ", i as u8, i as u8, i as u8)?;
        }
    }
    julia.fill_canvas(data);
    julia.display().expect("Could not render image")
}

#[test]
fn owl() {
    // let mut rng = rand::thread_rng();
    let mut owl = Canvas::new(500, 500, 255);
    owl.upper_left_system = true;
    let corrs: [i32; 640] = [
        140, 39, 157, 77, 136, 103, 136, 153, 143, 174, 135, 201, 142, 215, 139, 244, 154, 279,
        170, 325, 203, 341, 208, 352, 249, 363, 188, 384, 243, 378, 257, 389, 250, 363, 378, 482,
        357, 441, 379, 472, 377, 457, 357, 440, 377, 449, 373, 416, 331, 388, 373, 411, 356, 365,
        383, 404, 373, 321, 349, 286, 376, 318, 371, 257, 357, 240, 374, 256, 366, 225, 307, 158,
        307, 106, 286, 81, 302, 37, 261, 55, 221, 52, 181, 57, 140, 39, 139, 40, 168, 66, 178, 57,
        155, 79, 174, 90, 168, 67, 174, 90, 178, 57, 218, 75, 258, 56, 218, 75, 263, 93, 269, 68,
        299, 40, 284, 79, 258, 56, 263, 93, 284, 79, 277, 96, 262, 104, 250, 102, 219, 119, 218,
        76, 174, 92, 160, 95, 155, 78, 133, 104, 158, 96, 132, 130, 148, 153, 151, 106, 171, 114,
        176, 105, 154, 107, 150, 125, 172, 120, 163, 139, 178, 127, 176, 157, 188, 130, 206, 148,
        197, 127, 202, 116, 199, 106, 189, 102, 176, 106, 178, 105, 171, 111, 171, 120, 177, 127,
        185, 131, 193, 128, 217, 121, 219, 148, 230, 150, 240, 126, 249, 128, 231, 148, 262, 157,
        250, 131, 258, 127, 260, 156, 259, 127, 264, 123, 275, 140, 262, 127, 265, 120, 288, 126,
        269, 114, 287, 106, 267, 105, 251, 103, 240, 108, 237, 114, 240, 123, 221, 119, 175, 91,
        161, 95, 176, 105, 187, 115, 188, 105, 188, 115, 196, 107, 189, 114, 201, 116, 186, 114,
        198, 125, 188, 115, 188, 127, 188, 114, 182, 127, 188, 114, 173, 120, 186, 115, 173, 115,
        187, 116, 177, 107, 177, 107, 174, 89, 218, 77, 263, 92, 262, 104, 251, 114, 251, 102, 252,
        116, 243, 108, 252, 115, 237, 116, 248, 115, 238, 126, 249, 114, 248, 128, 252, 116, 256,
        129, 250, 112, 263, 122, 251, 117, 266, 114, 252, 114, 287, 125, 276, 140, 263, 154, 292,
        156, 289, 108, 277, 100, 307, 132, 307, 157, 293, 157, 335, 195, 270, 173, 293, 159, 263,
        155, 223, 167, 270, 175, 266, 200, 294, 199, 271, 175, 335, 196, 325, 226, 296, 200, 293,
        228, 268, 200, 268, 202, 295, 199, 334, 197, 352, 238, 326, 225, 349, 252, 351, 242, 347,
        282, 324, 251, 326, 226, 323, 279, 277, 251, 293, 228, 277, 251, 267, 200, 244, 217, 277,
        251, 243, 217, 221, 168, 178, 176, 221, 169, 178, 157, 144, 177, 179, 177, 156, 201, 143,
        212, 142, 179, 156, 202, 159, 242, 144, 216, 159, 243, 178, 225, 178, 177, 212, 199, 243,
        219, 252, 274, 221, 247, 213, 201, 219, 249, 182, 226, 178, 263, 189, 309, 154, 281, 159,
        244, 178, 265, 220, 301, 220, 251, 251, 274, 268, 326, 220, 301, 246, 362, 191, 309, 210,
        354, 246, 363, 252, 381, 241, 379, 214, 374, 245, 364, 315, 430, 293, 385, 245, 366, 294,
        387, 267, 329, 305, 342, 324, 385, 294, 386, 315, 427, 354, 439, 326, 386, 309, 344, 270,
        327, 253, 275, 293, 299, 322, 313, 336, 355, 322, 312, 345, 339, 348, 307, 323, 281, 322,
        316, 324, 252, 347, 282, 348, 308, 368, 344, 354, 366, 345, 341, 354, 365, 368, 346, 371,
        320, 349, 285, 349, 254, 351, 239, 338, 196, 339, 194, 326, 224, 324, 251, 293, 227, 278,
        253, 293, 300, 254, 277, 269, 328, 306, 344, 294, 299, 304, 345, 336, 359, 351, 362, 307,
        344, 321, 383, 297, 388, 249, 365, 218, 303, 219, 247, 213, 200, 177, 176, 220, 174, 218,
        152, 233, 150, 220, 118, 218, 75, 220, 50, 219, 77, 218, 117, 206, 151, 200, 127, 206, 152,
        191, 136, 207, 150, 181, 153, 223, 168, 179, 154, 178, 132, 178, 153, 149, 156, 149, 156,
        135, 158,
    ];

    owl.set_line_color(255, 255, 255);
    let chunks = 2;
    for i in (0..corrs.len()).step_by(chunks) {
        if i != corrs.len() - chunks {
            // owl.set_line_color(
            //     rng.gen_range(0..=255),
            //     rng.gen_range(0..=255),
            //     rng.gen_range(0..=255),
            // );
            // owl.save_binary(&format!("anim/owl{:04}.ppm", i))
            //     .expect("could not save image");
            owl.draw_line(
                owl.line,
                corrs[i] as f64,
                corrs[i + 1] as f64,
                corrs[i + 2] as f64,
                corrs[i + 3] as f64,
            )
        }
    }
    owl.display().expect("Could not display image")
    // owl.save_binary(&format!("anim/owl{:04}.ppm", 319))
    //     .expect("could not save image");
    // utils::animation("owl", "owl.gif");
    // utils::view_animation("owl.gif");
}
