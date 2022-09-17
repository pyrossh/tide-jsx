pub mod fragment;
pub mod html;
pub mod html_escaping;
mod numbers;
mod render;
mod simple_element;
mod text_element;

pub use self::render::Render;
pub use fragment::Fragment;
use tide::{http::mime, StatusCode};
pub use tide_jsx_impl::{component, html, rsx};
pub use simple_element::SimpleElement;
pub use text_element::Raw;

impl<'a, T: Render> From<SimpleElement<'a, T>> for tide::Response {
    fn from(s: SimpleElement<'a, T>) -> Self {
        tide::Response::builder(StatusCode::Ok)
            .content_type(mime::HTML)
            .body(s.render())
            .build()
    }
}

impl<'a, T: Render> From<SimpleElement<'a, T>> for tide::Result {
    fn from(s: SimpleElement<'a, T>) -> Self {
        Ok(s.into())
    }
}
