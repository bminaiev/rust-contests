use algo_lib::{
    geometry::point::PointT,
    misc::{ord_f64::OrdF64, rand::Random},
};

use crate::plotters_wrappers::save_plot;

type Point = PointT<OrdF64>;

pub struct DynamicPlot {
    points: Vec<Vec<Point>>,
    max_points: usize,
    pub description: String,
    pub x_name: String,
    pub y_name: String,
    rnd: Random,
    cur_stay_prob: Vec<f64>,
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
            cur_stay_prob: vec![],
        }
    }

    pub fn add_point_scecific_graph<T>(&mut self, x: T, y: T, graph: usize)
    where
        OrdF64: From<T>,
    {
        while graph >= self.points.len() {
            self.points.push(vec![]);
            self.cur_stay_prob.push(1.0);
        }
        let p = Point::new(x, y);
        if self.rnd.gen_double() < self.cur_stay_prob[graph] {
            self.points[graph].push(p);
        }
        if self.points[graph].len() > self.max_points {
            let mut new_points = vec![];
            for p in self.points[graph].iter() {
                if self.rnd.gen_bool() {
                    new_points.push(p.clone());
                }
            }
            self.cur_stay_prob[graph] /= 2.0;
            self.points[graph] = new_points;
        }
    }

    pub fn add_point<T>(&mut self, x: T, y: T)
    where
        OrdF64: From<T>,
    {
        self.add_point_scecific_graph(x, y, 0);
    }

    ///
    /// Returns full [file_prefix] + "." + extension of saved file
    ///
    pub fn save_image(&self, base_dir: &str, file_prefix: &str) -> String {
        save_plot(
            &self.points,
            base_dir,
            file_prefix,
            &self.x_name,
            &self.y_name,
        )
    }

    pub fn save_sub_image(&self, base_dir: &str, file_prefix: &str, graph_id: usize) -> String {
        save_plot(
            &[self.points[graph_id].clone()],
            base_dir,
            file_prefix,
            &self.x_name,
            &self.y_name,
        )
    }
}
