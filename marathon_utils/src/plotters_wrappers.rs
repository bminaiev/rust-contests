use std::ops::Range;

use algo_lib::{geometry::point::PointT, misc::ord_f64::OrdF64};

use plotters::prelude::*;

fn calc_range(coords: &[OrdF64]) -> Range<f64> {
    if coords.is_empty() {
        return 0.0..1.0;
    }
    let min_val = coords.iter().min().unwrap();
    let max_val = coords.iter().max().unwrap();
    min_val.0..max_val.0
}

const SIZE: u32 = 750;

pub fn save_plot(
    data: &[PointT<OrdF64>],
    base_dir: &str,
    file_prefix: &str,
    x_label: &str,
    y_label: &str,
) -> String {
    let path = format!("{}/{}.png", base_dir, file_prefix);
    let root = BitMapBackend::new(&path, (SIZE, SIZE)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let all_xs: Vec<_> = data.iter().map(|p| p.x).collect();
    let all_ys: Vec<_> = data.iter().map(|p| p.y).collect();

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .x_label_area_size(40u32)
        .y_label_area_size(50u32)
        .build_cartesian_2d(calc_range(&all_xs), calc_range(&all_ys))
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()
        .unwrap();

    chart
        .draw_series(
            data.iter()
                .map(|p| Circle::new((p.x.0, p.y.0), 1, BLUE.filled())),
        )
        .unwrap();

    format!("{}.png", file_prefix)
}
