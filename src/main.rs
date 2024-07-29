use plotters::coord::ranged1d::{LightPoints, Ranged};
use plotters::{backend::RGBPixel, coord::types::RangedCoordf64, prelude::*};

fn draw_circle(
    chart: &mut ChartContext<BitMapBackend<RGBPixel>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    x_coordinates: Vec<f64>,
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
        let bypotenuse = (second - first).abs() * 5f64 / 4f64;
        if first < second {
            Self {
                left_focus: Focus { x: first },
                right_focus: Focus { x: second },
                mid_focus_point,
                bypotenuse,
            }
        } else {
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
    // The base of a right triangle with hypotenuse
    // contributing to the bypotenuse
    fn base(&self, focus: Focus, x_coord: f64) -> f64 {
        (focus.x - x_coord).abs()
    }
    // first arg hypotenuse
    fn first_hypotenuse(&self, first_base: f64, second_base: f64) -> f64 {
        let numerator = self.bypotenuse.powf(2.0) + first_base.powf(2.0) - second_base.powf(2.0);
        let denominator = 2.0 * self.bypotenuse;
        numerator / denominator
    }
    fn y_from_h_and_x(h: f64, x: f64) -> f64 {
        (h.powf(2.0) - x.powf(2.0)).sqrt()
    }
    fn calculate_curve_y(&self, x_coord: f64) -> f64 {
        let left_base = self.base(self.left_focus, x_coord);
        let right_base = self.base(self.right_focus, x_coord);
        let h_l = self.first_hypotenuse(left_base, right_base);
        let y = Ellipse::y_from_h_and_x(h_l, left_base);
        if y > 0f64 {
            y
        } else {
            0f64
        }
    }
}
// Draw operations
impl Ellipse {
    fn draw_curve(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) {
        let rc = RangedCoordf64::from(
            (self.right_focus.x - self.bypotenuse)..(self.left_focus.x + self.bypotenuse),
        )
        .key_points(LightPoints::new(1, 100));
        let series = rc.iter().map(|x| (*x, self.calculate_curve_y(*x))).chain(
            rc.iter()
                .rev()
                .map(|x| (*x, -1f64 * self.calculate_curve_y(*x))),
        );
        let shape_style = ShapeStyle {
            color: full_palette::PURPLE.to_rgba(),
            filled: true,
            stroke_width: 1,
        };
        chart
            .draw_series(LineSeries::new(series, shape_style))
            .unwrap();
    }
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
    fn draw_mid_focus_point(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) {
        chart
            .draw_series(vec![Circle::new((self.mid_focus_point, 0f64), 5, &BLUE)])
            .unwrap();
    }
    fn draw_first_flat_bypotenuse(
        &self,
        chart: &mut ChartContext<
            BitMapBackend<RGBPixel>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) {
        let rc = RangedCoordf64::from(self.left_focus.x..(self.left_focus.x + self.bypotenuse))
            .key_points(LightPoints::new(1, 100));
        let series = rc.iter().map(|x| (x.clone(), 0f64));
        chart.draw_series(LineSeries::new(series, &BLACK)).unwrap();
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

    let rc_coordinates =
        RangedCoordf64::from(-3.14f64..3.14f64).key_points(LightPoints::new(1, 900));
    let shape_style = ShapeStyle {
        color: RED.to_rgba(),
        filled: true,
        stroke_width: 5,
    };
    draw_circle(&mut chart, rc_coordinates, 3.14, shape_style);
    ellipse.draw_foci(&mut chart);
    dbg!(ellipse.left_focus.x);
    dbg!(ellipse.mid_focus_point);
    ellipse.draw_mid_focus_point(&mut chart);
    ellipse.draw_first_flat_bypotenuse(&mut chart);
    dbg!(ellipse.right_focus.x);
    dbg!(ellipse.bypotenuse);
    ellipse.draw_curve(&mut chart);
}
