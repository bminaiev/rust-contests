use image::{ImageBuffer, Rgb};
use plotlib::{
    page::Page,
    repr::{Histogram, HistogramBins},
    style::BoxStyle,
    view::ContinuousView,
};
use std::{fmt::Display, fs};

use crate::distribution_stat::DistributionStat;

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
}

pub struct HtmlReport {
    base_dir: String,
    prefix: String,
    relative_path: String,
    elements: Vec<Element>,
    uniq_id: usize,
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

    pub fn add_distribution_stat<T: Ord + Clone>(&mut self, stat: &DistributionStat<T>)
    where
        f64: From<T>,
    {
        let img_name = self.gen_uniq_name("svg");
        let data = stat.f64_data();
        let h = Histogram::from_slice(&data, HistogramBins::Count(20))
            .style(&BoxStyle::new().fill("burlywood"));
        let v = ContinuousView::new().add(h);
        Page::single(&v)
            .save(&format!("{}/{}", self.base_dir, img_name))
            .expect("saving svg");

        self.elements.push(Element::Image(Image::new(
            stat.name.clone(),
            img_name.clone(),
        )));
    }

    pub fn add_link(&mut self, text: &str, link: &str) {
        self.elements.push(Element::Link(Link {
            text: text.to_string(),
            link: link.to_string(),
        }));
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
            match element {
                Element::Link(link) => {
                    let mut a = div.a().attr(&format!("href='{}'", link.link));
                    write!(a, "{}", link.text)?
                }
                Element::Text(text) => {
                    write!(div, "{}", text)?;
                }
                Element::Image(image) => {
                    write!(div.div(), "{}", image.desciption)?;
                    let mut a = div
                        .a()
                        .attr(&format!("href='{}'", image.path))
                        .attr("target=_blank");
                    a.img()
                        .attr(&format!("src='{}'", image.path))
                        .attr("width=500")
                        .attr("style=\"image-rendering:pixelated;\"");
                }
            }
        }

        body.hr();

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
