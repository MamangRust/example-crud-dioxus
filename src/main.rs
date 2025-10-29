mod models;
mod routes;
mod components;
mod pages;

use dioxus::prelude::*;
use models::{Todo,  get_next_id};
use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let logged_in = use_signal(|| false);
    let todos = use_signal(|| {
        vec![
            Todo {
                id: get_next_id(),
                title: "Learn Dioxus".to_string(),
                description: "Understand the basics of Dioxus framework".to_string(),
            },
            Todo {
                id: get_next_id(),
                title: "Build CRUD App".to_string(),
                description: "Create a complete CRUD application".to_string(),
            },
        ]
    });

    use_context_provider(|| logged_in);
    use_context_provider(|| todos);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
