use curves_rs::graphics::display::*;
use curves_rs::graphics::matrix::*;
// use curves_rs::utils;

#[test]
// #[should_panic]
fn geass() {
    let mut img = Canvas::new_with_bg(800, 800, 255, &Pixel::new(24, 26, 27));
    let mut geass = Matrix::new(4, 0, Vec::with_capacity(112 * 2));
    img.upper_left_system = true;

    let geass_corrs = [
        170, 216, 190, 249, 190, 249, 220, 274, 220, 274, 250, 295, 250, 295, 289, 318, 289, 318,
        347, 349, 347, 349, 347, 421, 347, 421, 400, 449, 400, 449, 453, 421, 453, 421, 453, 349,
        453, 349, 511, 318, 511, 318, 550, 295, 550, 295, 580, 274, 580, 274, 606, 249, 606, 249,
        630, 216, 630, 216, 601, 285, 601, 285, 571, 323, 571, 323, 525, 358, 525, 358, 492, 388,
        492, 388, 489, 448, 489, 448, 441, 475, 441, 475, 400, 499, 400, 499, 359, 475, 359, 475,
        311, 448, 311, 448, 308, 388, 308, 388, 275, 358, 275, 358, 229, 323, 229, 323, 199, 285,
        199, 285, 170, 216,
    ];

    for corr in geass_corrs.chunks(2) {
        geass.add_point(corr[0] as f64, corr[1] as f64, 0.0)
    }

    let mut base = Matrix::translate(-400.0, -400.0, 0.0);
    let mut reflect = Matrix::reflect_xz();
    let mut half = Matrix::reflect_45();
    let mut last_half = Matrix::reflect_yz();

    base *= geass.clone();
    reflect *= base.clone();
    last_half *= half.clone();
    last_half *= base.clone();
    half *= base.clone();
    base.add_dataset(&reflect);
    base.add_dataset(&half);
    base.add_dataset(&last_half);

    let white = Pixel::new(255, 255, 255);
    img.set_line_pixel(&white);
    img.draw_lines(
        &geass
            .mult_matrix(&Matrix::scale(0.1, 0.1, 0.1))
            .mult_matrix(&Matrix::translate(360.0, 370.0, 0.0)),
    );
    img.fill(406, 413, &white, &white);
    img.set_line_pixel(&Pixel::new(191, 70, 61));
    for i in 0..180 {
        let mut copy = img.clone();
        copy.draw_lines(&base.mult_matrix(
            &Matrix::rotate_y(i as f64).mult_matrix(&Matrix::translate(400.0, 400.0, 0.0)),
        ));
        // copy.save_extension(&format!("anim/geass{:04}.png", i))
        //     .expect("Could not save image")
    }
    img.display().expect("Could not display image")
    // let file_name = "./geass.gif";
    // utils::animation("geass", file_name);
    // utils::view_animation(file_name)
}
