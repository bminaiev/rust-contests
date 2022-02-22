use algo_lib::{
    geometry::point::PointT,
    misc::{ord_f64::OrdF64, rand::Random},
};

use crate::plotlib_wrappers::save_plot;

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

    ///
    /// Returns full [file_prefix] + "." + extension of saved file
    ///
    pub fn save_image(&self, base_dir: &str, file_prefix: &str) -> String {
        save_plot(
            self.points.iter().map(|x| *x),
            base_dir,
            file_prefix,
            &self.x_name,
            &self.y_name,
        )
    }
}
