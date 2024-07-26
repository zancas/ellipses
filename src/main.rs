use plotters::prelude::*;
fn half_circle(radius: f64, x_coord: f64) -> f64 {
    // r_sq = x_sq + y_sq -> y_sq = r_sq - x_sq ->
    // y = (r_sq - x_sq)^.5
    (radius.powf(2f64) - x_coord.powf(2f64)).sqrt()
}

fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.1.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
            &RED,
        ))
        .unwrap();
}
