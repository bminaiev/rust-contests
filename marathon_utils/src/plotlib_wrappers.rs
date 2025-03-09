use algo_lib::{
    geometry::{bounding_box::BoundingBox, point::PointT},
    misc::{num_traits::HasConstants, ord_f64::OrdF64},
};
use plotlib::{page::Page, repr::Plot, style::PointStyle, view::ContinuousView};

pub fn save_plot(
    data: impl Iterator<Item = PointT<OrdF64>>,
    base_dir: &str,
    file_prefix: &str,
    x_label: &str,
    y_label: &str,
) -> String {
    let mut data: Vec<(f64, f64)> = data.map(|p| (p.x.0, p.y.0)).collect();
    let should_add_fake_points = if data.len() <= 1 {
        true
    } else {
        let first_point: PointT<OrdF64> = PointT::new(data[0].0, data[0].1);
        let mut bbox = BoundingBox::new(&first_point, &first_point);
        for &(x, y) in data.iter() {
            bbox.add(&PointT::new(x, y));
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
        .x_label(x_label)
        .y_label(y_label);

    Page::single(&view)
        .save(&format!("{}/{}.svg", base_dir, file_prefix))
        .expect("saving svg");

    format!("{}.svg", file_prefix)
}
