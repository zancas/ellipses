use plotters::{backend::RGBPixel, coord::types::RangedCoordf64, prelude::*};

fn draw_circle(
    chart: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    x_coordinates: &[f64],
    radius: f64,
    line_shape_style: ShapeStyle,
) {
    let series = x_coordinates
        .iter()
        .map(|x| (*x, circle_pos_y_coord(radius, *x)))
        .chain(
            x_coordinates
                .iter()
                .rev()
                .map(|x| (*x, -1f64 * circle_pos_y_coord(radius, *x))),
        );
    chart
        .draw_series(LineSeries::new(series, line_shape_style))
        .unwrap();
}
fn circle_pos_y_coord(radius: f64, x_coord: f64) -> f64 {
    // r_sq = x_sq + y_sq -> y_sq = r_sq - x_sq ->
    // y = (r_sq - x_sq)^.5
    let y = (radius.powf(2f64) - x_coord.powf(2f64)).sqrt();
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

fn place_foci(lower_bound: f64, upper_bound: f64) -> Vec<Focus> {
    use rand::Rng;
    vec![
        Focus {
            x: rand::thread_rng().gen_range(lower_bound..upper_bound),
        },
        Focus {
            x: rand::thread_rng().gen_range(lower_bound..upper_bound),
        },
    ]
}

struct Ellipse {
    focus_a: Focus,
    focus_b: Focus,
    bypotenuse: f64,
}

#[derive(Clone, Copy)]
struct Focus {
    x: f64,
}
fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.1.png", (900, 900)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -3.14..3.14)
        .unwrap();

    let x_coordinates: Vec<f64> = (-315..315).map(|x| x as f64 / 100.0).collect();
    // The radius will be divided by 100
    // after each size 1 step.
    let shape_style = ShapeStyle {
        color: GREEN.to_rgba(),
        filled: true,
        stroke_width: 5,
    };
    draw_circle(&mut chart, &x_coordinates, 3.14, shape_style);
    chart
        .draw_series(LineSeries::new(
            (-290..290).map(|x| x as f64 / 100.0).map(|x| (x, 0f64)),
            &BLACK,
        ))
        .unwrap();
    let foci = place_foci(-2.04, 2.04);
    chart
        .draw_series(foci.iter().map(|&f| Circle::new((f.x, 0f64), 5, &RED)))
        .unwrap();
}
