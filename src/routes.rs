use dioxus::prelude::*;
use crate::components::Navbar;
use crate::pages::{Login, Home, Create, Edit};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/create")]
    Create {},
    #[route("/edit/:id")]
    Edit { id: usize },
}
