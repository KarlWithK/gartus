use transform_rs::graphics::display::*;
use transform_rs::graphics::matrix::*;
use transform_rs::utils;

#[test]
pub fn make_pory() {
    let outline = Pixel::new(235, 219, 178);
    let white = Pixel::new(243, 244, 248);
    let purplish = Pixel::new(17, 46, 81);
    let light_red = Pixel::new(255, 76, 80);
    let dark_red = Pixel::new(191, 70, 61);
    let dark_blue = Pixel::new(69, 175, 214);
    let light_blue = Pixel::new(92, 216, 252);
    let mut porygon = Canvas::new_with_bg(512, 512, 255, purplish);
    let mut matrix = Matrix::new(0, 4, Vec::new());
    let mut dilate = Matrix::scale(0.5, 0.5, 0.5);
    let mut translate = Matrix::translate(0.0, 45.0, 0.0);
    porygon.upper_left_system = true;
    let corrs = [
        552, 661, 622, 654, 622, 654, 768, 535, 768, 535, 743, 505, 743, 505, 669, 510, 669, 510,
        604, 621, 604, 621, 622, 654, 622, 654, 604, 621, 604, 621, 540, 607, 540, 607, 552, 661,
        552, 661, 540, 607, 540, 607, 614, 473, 614, 473, 669, 512, 669, 512, 604, 621, 604, 621,
        669, 510, 669, 510, 743, 505, 743, 505, 694, 471, 694, 471, 614, 473, 565, 545, 522, 552,
        522, 552, 406, 557, 406, 557, 337, 470, 337, 470, 367, 382, 367, 382, 490, 374, 490, 374,
        552, 342, 552, 342, 691, 402, 691, 402, 716, 453, 716, 453, 716, 481, 716, 481, 565, 545,
        565, 545, 522, 552, 522, 552, 506, 468, 506, 468, 337, 470, 337, 470, 506, 468, 506, 468,
        716, 453, 716, 453, 506, 468, 506, 468, 530, 431, 530, 431, 490, 374, 490, 374, 506, 468,
        367, 382, 490, 374, 490, 374, 552, 342, 552, 342, 592, 312, 592, 312, 614, 255, 614, 255,
        555, 176, 555, 176, 459, 166, 459, 166, 399, 194, 399, 194, 349, 233, 349, 233, 313, 282,
        313, 282, 285, 384, 285, 384, 327, 384, 327, 384, 367, 382, 614, 255, 538, 246, 538, 246,
        515, 300, 515, 300, 463, 287, 463, 287, 436, 250, 436, 250, 463, 287, 463, 287, 417, 305,
        417, 305, 334, 384, 334, 384, 322, 376, 322, 376, 287, 375, 287, 375, 322, 376, 322, 376,
        386, 290, 386, 290, 436, 250, 436, 250, 456, 206, 456, 206, 503, 204, 503, 204, 538, 246,
        538, 246, 503, 204, 503, 204, 555, 176, 456, 376, 417, 305, 417, 305, 386, 290, 386, 290,
        313, 282, 490, 243, 505, 246, 505, 246, 493, 263, 493, 263, 470, 256, 470, 256, 475, 229,
        475, 229, 495, 233, 495, 233, 490, 243, 672, 392, 802, 292, 802, 292, 814, 287, 814, 287,
        827, 305, 827, 305, 809, 302, 809, 302, 802, 292, 802, 292, 809, 302, 809, 302, 702, 424,
        702, 424, 809, 302, 809, 302, 827, 305, 827, 305, 716, 478, 337, 470, 293, 481, 293, 481,
        241, 480, 241, 480, 337, 470, 337, 470, 293, 481, 293, 481, 280, 517, 280, 517, 423, 607,
        423, 607, 280, 517, 280, 517, 221, 520, 221, 520, 241, 480, 241, 480, 221, 520, 221, 520,
        354, 611, 354, 611, 423, 607, 423, 607, 451, 575, 451, 575, 443, 560, 443, 560, 337, 470,
    ];
    for corr in corrs.chunks(2) {
        matrix.add_point(corr[0] as f64, corr[1] as f64, 0.0);
    }
    dilate *= matrix;
    translate *= dilate;
    let filename = "porygon";
    porygon.set_line_pixel(outline);
    porygon.draw_lines_for_animation(&translate, filename);
    porygon.draw_line(dark_blue, 221.5, 325.0, 168.5, 280.0);
    let fill_colors: Vec<Pixel> = vec![
        light_blue, dark_blue, light_blue, light_blue, light_blue, light_blue, dark_red, dark_blue,
        dark_blue, dark_blue, dark_blue, dark_blue, light_blue, light_blue, light_blue, light_blue,
        light_red, dark_blue, dark_blue, light_blue, dark_red, dark_red, light_red, light_red,
        dark_blue, dark_blue, light_blue, light_blue, white,
    ];
    let fill_points = [
        284, 365, 336, 330, 301, 329, 306, 297, 332, 290, 349, 295, 269, 304, 227, 309, 172, 307,
        198, 313, 160, 330, 146, 284, 128, 293, 146, 284, 210, 257, 254, 261, 299, 263, 374, 246,
        405, 194, 353, 237, 245, 214, 210, 183, 275, 153, 202, 156, 206, 227, 189, 207, 164, 214,
        150, 235, 259, 177, 242,
    ];
    for (vector, color) in fill_points.chunks(2).zip(fill_colors) {
        porygon.fill_with_animation(vector[0], vector[1], color, outline, filename)
    }
    porygon
        .save_binary(&format!("anim/{}{:08}.ppm", filename, 100000))
        .expect("Could not save to file");
    utils::animation("porygon", "porygon.gif");
}