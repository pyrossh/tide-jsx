use crate::Render;
use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct Fragment<T: Render> {
    pub children: T,
}

impl<T: Render> Render for Fragment<T> {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        self.children.render_into(writer)
    }
}
