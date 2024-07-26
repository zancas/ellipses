use plotters::{backend::RGBPixel, coord::types::RangedCoordf64, prelude::*};

fn draw_circle(
    chart: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    radius: i32,
    _x_center: f64,
) {
    dbg!(radius);
    let scaled_radius = radius as f64 / 100.0;
    chart
        .draw_series(LineSeries::new(
            (-radius..radius)
                .map(|x| x as f64 / 100.0)
                .map(|x| (x, circle_pos_y_coord(scaled_radius, x))),
            &RED,
        ))
        .unwrap();
    chart
        .draw_series(LineSeries::new(
            (-radius..radius)
                .map(|x| x as f64 / 100.0)
                .map(|x| (x, -1f64 * circle_pos_y_coord(scaled_radius, x))),
            &BLUE,
        ))
        .unwrap();
}
fn circle_pos_y_coord(radius: f64, x_coord: f64) -> f64 {
    // r_sq = x_sq + y_sq -> y_sq = r_sq - x_sq ->
    // y = (r_sq - x_sq)^.5
    let y = (radius.powf(2f64) - x_coord.powf(2f64)).sqrt();
    dbg!(y);
    if y > 0f64 {
        y
    } else {
        0f64
    }
}
fn half_ellipse(semi_major_axis: f64) -> f64 {
    unimplemented!(
        "Make two circles, one centered at each focus.  The circles intersect once
        above the x-axis, on the curve."
    )
}

fn place_foci(lower_bound: f64, upper_bound: f64) -> Vec<f64> {
    use rand::Rng;
    vec![
        rand::thread_rng().gen_range(lower_bound..upper_bound),
        rand::thread_rng().gen_range(lower_bound..upper_bound),
    ]
}
fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.1.png", (900, 900)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -3.14..3.14)
        .unwrap();

    // The radius will be divided by 100
    // after each size 1 step.
    draw_circle(&mut chart, 314, 0f64);
    chart
        .draw_series(LineSeries::new(
            (-290..290).map(|x| x as f64 / 100.0).map(|x| (x, 0f64)),
            &BLACK,
        ))
        .unwrap();
    let foci = place_foci(-3.14, 3.14);
    chart
        .draw_series(foci.iter().map(|&f| Circle::new((f, 0f64), 5, &RED)))
        .unwrap();
}
