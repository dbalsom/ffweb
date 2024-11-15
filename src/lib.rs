#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub(crate) mod worker;

pub use app::TemplateApp;
