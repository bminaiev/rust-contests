use std::{cmp::min, ops::Range};

use algo_lib::{geometry::point::PointT, misc::ord_f64::OrdF64};

use plotters::prelude::*;

use crate::distribution_stat::DistributionStat;

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

// TODO: make it look nice :(
pub fn save_distribution_stat(
    data: &DistributionStat<i32>,
    base_dir: &str,
    file_prefix: &str,
    x_label: &str,
    y_label: &str,
) -> String {
    let path = format!("{}/{}.png", base_dir, file_prefix);
    let root = BitMapBackend::new(&path, (SIZE, SIZE)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let data: Vec<i32> = data.data().iter().cloned().collect();

    let range = *data.iter().min().unwrap()..*data.iter().max().unwrap() + 1;

    // TODO: smarted logic
    let buckets: usize = 20;

    let id = |value: i32| -> usize {
        let id = (value - range.start) as i64 * (buckets as i64) / (range.end - range.start) as i64;
        min(buckets - 1, id as usize)
    };

    let mut counts = vec![0; buckets];
    for &x in data.iter() {
        counts[id(x)] += 1;
    }
    let max_count = *counts.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .x_label_area_size(40u32)
        .y_label_area_size(50u32)
        .build_cartesian_2d(0..buckets as i32, 0..max_count)
        .unwrap();

    let histogram = Histogram::vertical(&chart)
        .style(BLUE.filled())
        // .margin(0)
        .data(data.iter().map(|&value| (id(value) as i32, 1)));

    chart.draw_series(histogram).unwrap();

    let offset = |bucket_id: i32| -> i32 {
        min(
            range.end,
            range.start
                + ((bucket_id as i64) * (range.end - range.start) as i64 / (buckets as i64)) as i32,
        )
    };

    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .x_label_formatter(&|&bucket_id| format!("{}", offset(bucket_id)))
        .draw()
        .unwrap();

    format!("{}.png", file_prefix)
}
