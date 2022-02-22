use algo_lib::{
    geometry::{bounding_box::BoundingBox, point::PointT},
    misc::{num_traits::HasConstants, ord_f64::OrdF64, rand::Random},
};
use plotlib::{page::Page, repr::Plot, style::PointStyle, view::ContinuousView};

type Point = PointT<OrdF64>;

pub struct DynamicPlot {
    points: Vec<Point>,
    max_points: usize,
    pub description: String,
    pub x_name: String,
    pub y_name: String,
    rnd: Random,
    cur_stay_prob: f64,
}

impl DynamicPlot {
    pub fn new(description: &str, x_name: &str, y_name: &str) -> Self {
        Self {
            points: vec![],
            max_points: 2_000,
            description: description.to_owned(),
            x_name: x_name.to_owned(),
            y_name: y_name.to_owned(),
            rnd: Random::new(787788),
            cur_stay_prob: 1.0,
        }
    }

    pub fn add_point<T, U>(&mut self, x: T, y: U)
    where
        OrdF64: From<T>,
        OrdF64: From<U>,
    {
        let p = Point::new(x.into(), y.into());
        if self.rnd.gen_double() < self.cur_stay_prob {
            self.points.push(p);
        }
        if self.points.len() > self.max_points {
            let mut new_points = vec![];
            for p in self.points.iter() {
                if self.rnd.gen_bool() {
                    new_points.push(p.clone());
                }
            }
            self.cur_stay_prob /= 2.0;
            self.points = new_points;
        }
    }

    pub fn gen_image(&self, base_dir: &str, file_prefix: &str) -> String {
        let mut data: Vec<(f64, f64)> = self.points.iter().map(|p| (p.x.0, p.y.0)).collect();
        let should_add_fake_points = if data.len() <= 1 {
            true
        } else {
            let mut bbox = BoundingBox::new(&self.points[0], &self.points[0]);
            for p in self.points.iter() {
                bbox.add(p);
            }
            bbox.dx() == OrdF64::ZERO || bbox.dy() == OrdF64::ZERO
        };
        if should_add_fake_points {
            data.push((0.0, 0.0));
            data.push((1.0, 1.0));
        }

        let plot: Plot = Plot::new(data).point_style(PointStyle::new().size(1.0));

        let view = ContinuousView::new()
            .add(plot)
            .x_label(&self.x_name)
            .y_label(&self.y_name);

        Page::single(&view)
            .save(&format!("{}/{}.svg", base_dir, file_prefix))
            .expect("saving svg");

        format!("{}.svg", file_prefix)
    }
}