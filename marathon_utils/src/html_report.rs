use image::{ImageBuffer, Rgb};
use std::{fmt::Display, fs};

pub type ImageData = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub struct Image {
    name: String,
    data: ImageData,
}

impl Image {
    pub fn new(name: String, data: ImageData) -> Self {
        Self { name, data }
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
}

impl HtmlReport {
    pub fn new(base_dir: String, prefix: String, relative_path: String) -> Self {
        Self {
            base_dir,
            prefix,
            relative_path,
            elements: vec![],
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.elements.push(Element::Text(text.to_string()));
    }

    pub fn add_value<V: Display>(&mut self, name: &str, value: &V) {
        self.elements
            .push(Element::Text(format!("{}: {}", name, value)))
    }

    pub fn add_image(&mut self, name: &str, image: ImageData) {
        self.elements
            .push(Element::Image(Image::new(name.to_string(), image)));
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
                    let name = format!("{}{}", self.prefix, image.name);
                    image
                        .data
                        .save(&format!("{}/{}", self.base_dir, name))
                        .expect("Can't save image :(");
                    let mut a = div
                        .a()
                        .attr(&format!("href='{}'", name))
                        .attr("target=_blank");
                    a.img()
                        .attr(&format!("src='{}'", name))
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
