pub mod fragment;
pub mod html;
pub mod html_escaping;
mod numbers;
mod render;
mod simple_element;
mod text_element;

pub use self::render::Render;
pub use fragment::Fragment;
pub use tide_jsx_impl::{component, html, rsx};
pub use simple_element::SimpleElement;
pub use text_element::Raw;

impl<'a, T: Render> From<SimpleElement<'a, T>> for tide::Response {
    fn from(s: SimpleElement<T>) -> Self {
        tide::Body::from_string(s.render()).into()
    }
}
