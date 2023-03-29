// src/svg.rs
use svg::node::element::path::Data;

pub struct SVGPath {
    pub data: Data,
    pub style: String,
}

impl SVGPath {
    pub fn to_node(&self) -> svg::node::element::Path {
        let mut path = svg::node::element::Path::new();
        path.set("d", self.data.clone())
            .set("style", self.style.clone());
        path
    }
}

pub struct SVGImage {
    pub width: u32,
    pub height: u32,
    pub paths: Vec<SVGPath>,
}
