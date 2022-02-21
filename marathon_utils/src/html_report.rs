use algo_lib::misc::ord_f64::OrdF64;
use image::{ImageBuffer, Rgb};
use plotlib::{
    page::Page,
    repr::{Histogram, HistogramBins},
    style::BoxStyle,
    view::ContinuousView,
};
use std::{fmt::Display, fs};

use crate::{
    distribution_stat::DistributionStat, dynamic_plot::DynamicPlot, hashcode_solver::OneTest,
};

pub type ImageData = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub struct Image {
    desciption: String,
    path: String,
}

impl Image {
    pub fn new(desciption: String, path: String) -> Self {
        Self { desciption, path }
    }
}

pub struct Link {
    pub text: String,
    pub link: String,
}

pub enum Element {
    Text(String),
    Image(Image),
    Link(Link),
    Hr(),
    DynamicPlot(String, DynamicPlot),
}

pub struct HtmlReport {
    base_dir: String,
    prefix: String,
    relative_path: String,
    elements: Vec<Element>,
    uniq_id: usize,
}

#[derive(Clone, Copy)]
pub struct DynamicPlotId(usize);

impl DynamicPlotId {
    pub fn add_point<T, U>(&self, test: &mut OneTest, x: T, y: U)
    where
        OrdF64: From<T>,
        OrdF64: From<U>,
    {
        test.report.get_dynamic_plot(*self).add_point(x, y);
    }
}

impl HtmlReport {
    pub fn new(base_dir: String, prefix: String, relative_path: String) -> Self {
        Self {
            base_dir,
            prefix,
            relative_path,
            elements: vec![],
            uniq_id: 0,
        }
    }

    pub fn gen_uniq_name(&mut self, suffix: &str) -> String {
        self.uniq_id += 1;
        format!("{}{}.{}", self.prefix, self.uniq_id, suffix)
    }

    pub fn add_text(&mut self, text: &str) {
        self.elements.push(Element::Text(text.to_string()));
    }

    pub fn add_value<V: Display>(&mut self, name: &str, value: &V) {
        self.elements
            .push(Element::Text(format!("{}: {}", name, value)))
    }

    pub fn add_image(&mut self, name: &str, image: ImageData) {
        let full_name = format!("{}{}", self.prefix, name);
        image
            .save(&format!("{}/{}", self.base_dir, full_name))
            .expect("Can't save image :(");

        self.elements
            .push(Element::Image(Image::new(name.to_owned(), full_name)));
    }

    fn image_by_continius_view(
        &self,
        img_name: &str,
        description: &str,
        view: &ContinuousView,
    ) -> Image {
        Page::single(view)
            .save(&format!("{}/{}", self.base_dir, img_name))
            .expect("saving svg");

        Image::new(description.to_owned(), img_name.to_owned())
    }

    pub fn add_distribution_stat<T: Ord + Clone>(&mut self, stat: &DistributionStat<T>)
    where
        f64: From<T>,
    {
        let data = stat.f64_data();
        let h = Histogram::from_slice(&data, HistogramBins::Count(20))
            .style(&BoxStyle::new().fill("burlywood"));
        let v = ContinuousView::new().add(h);
        let img_name = self.gen_uniq_name("svg");
        let elem = self.image_by_continius_view(&img_name, &stat.name, &v);
        self.elements.push(Element::Image(elem));
    }

    pub fn add_dynamic_plot(&mut self, plot: DynamicPlot) -> DynamicPlotId {
        let img_name = self.gen_uniq_name("svg");
        self.elements.push(Element::DynamicPlot(img_name, plot));
        DynamicPlotId(self.elements.len() - 1)
    }

    pub fn get_dynamic_plot(&mut self, id: DynamicPlotId) -> &mut DynamicPlot {
        match &mut self.elements[id.0] {
            Element::DynamicPlot(_name, plot) => plot,
            _ => panic!("BUG in code!"),
        }
    }

    pub fn add_link(&mut self, text: &str, link: &str) {
        self.elements.push(Element::Link(Link {
            text: text.to_string(),
            link: link.to_string(),
        }));
    }

    pub fn add_hr(&mut self) {
        self.elements.push(Element::Hr());
    }

    // TODO: do not save too often
    pub fn save(&self) -> std::fmt::Result {
        use html_builder::*;
        use std::fmt::Write;

        let mut buf = Buffer::new();
        buf.doctype();
        let mut html = buf.html().attr("lang='en'");
        let mut head = html.head();
        head.meta().attr("charset='utf-8'");
        let mut body = html.body();

        for element in self.elements.iter() {
            let mut div = body.div();
            let handle_image = |div: &mut Node, image: &Image| -> std::fmt::Result {
                write!(div.div(), "{}", image.desciption)?;
                let mut a = div
                    .a()
                    .attr(&format!("href='{}'", image.path))
                    .attr("target=_blank");
                a.img()
                    .attr(&format!("src='{}'", image.path))
                    .attr("width=1000")
                    .attr("style=\"image-rendering:pixelated;\"");
                Ok(())
            };
            match element {
                Element::Link(link) => {
                    let mut a = div.a().attr(&format!("href='{}'", link.link));
                    write!(a, "{}", link.text)?
                }
                Element::Text(text) => {
                    write!(div, "{}", text)?;
                }
                Element::Image(image) => {
                    handle_image(&mut div, image)?;
                }
                Element::Hr() => {
                    body.hr();
                }
                Element::DynamicPlot(img_name, plot) => {
                    let image = self.image_by_continius_view(
                        img_name,
                        &plot.description,
                        &plot.gen_image(),
                    );
                    handle_image(&mut div, &image)?;
                }
            }
        }

        // Text contents in an inner node
        let mut footer = body.footer();
        writeln!(footer, "Last modified")?;
        let date = chrono::Local::now();
        writeln!(footer.time(), "{}", date.format("%Y-%m-%d %H:%M:%S"))?;

        // Finally, call finish() to extract the buffer.
        let data = buf.finish();
        fs::write(&format!("{}/{}", self.base_dir, self.relative_path), data)
            .expect("Can't save html report");
        Ok(())
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
}
