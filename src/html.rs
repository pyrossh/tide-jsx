use crate::Render;
use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct HTML5Doctype;

impl Render for HTML5Doctype {
    fn render_into<W: Write>(self, writer: &mut W) -> Result {
        write!(writer, "<!DOCTYPE html>")
    }
}
