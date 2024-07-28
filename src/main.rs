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
// Note that the major axis of this ellipse lies along the x-axis.
struct Ellipse {
    left_focus: Focus,
    right_focus: Focus,
    mid_focus_point: f64,
    bypotenuse: f64,
}
impl Ellipse {
    fn generate_random_ellipse(lower_bound: f64, upper_bound: f64) -> Self {
        use rand::thread_rng;
        use rand::Rng;
        let range = upper_bound - lower_bound;
        let minimum_interfocus = range / 10.0;
        let first = thread_rng().gen_range(lower_bound..upper_bound);
        let second = if thread_rng().gen_range(0..1) as f64 > 0.5 {
            thread_rng().gen_range(first + minimum_interfocus..upper_bound)
        } else {
            thread_rng().gen_range(lower_bound..first - minimum_interfocus)
        };
        let mid_focus_point = (first + second) / 2.0;
        if first < second {
            let bypotenuse = (second - first) * 5f64 / 4f64;
            Self {
                left_focus: Focus { x: first },
                right_focus: Focus { x: second },
                mid_focus_point,
                bypotenuse,
            }
        } else {
            let bypotenuse = (first - second) * 5f64 / 4f64;
            Self {
                left_focus: Focus { x: second },
                right_focus: Focus { x: first },
                mid_focus_point,
                bypotenuse,
            }
        }
    }
}
impl Ellipse {
    fn _base(focus: Focus, x_coord: f64) -> f64 {
        (focus.x - x_coord).abs()
    }
    /*
    fn _generate_on_curve_coordinate(_x_coord: f64) -> CurvePoint {
        todo!()
    }*/
    fn _calculate_curve_y(&self, _x_coord: f64) -> f64 {
        todo!()
    }
}
// Draw operations
impl Ellipse {
    fn draw_foci(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) {
        chart
            .draw_series(vec![
                Circle::new((self.left_focus.x, 0f64), 5, &RED),
                Circle::new((self.right_focus.x, 0f64), 5, &RED),
            ])
            .unwrap();
    }
}
#[derive(Clone, Copy)]
struct Focus {
    x: f64,
}
/*
#[derive(Clone, Copy)]
struct CurvePoint {
    x: f64,
    y: f64,
}*/
fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.1.png", (900, 900)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let ellipse = Ellipse::generate_random_ellipse(-2.04, 2.04);
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
    ellipse.draw_foci(&mut chart);
    dbg!(ellipse.left_focus.x);
    dbg!(ellipse.mid_focus_point);
    dbg!(ellipse.right_focus.x);
    dbg!(ellipse.bypotenuse);
}
