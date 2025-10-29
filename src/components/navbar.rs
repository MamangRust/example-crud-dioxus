use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Navbar() -> Element {
    let mut logged_in = use_context::<Signal<bool>>();
    let navigator = use_navigator();

    let handle_logout = move |_| {
        *logged_in.write() = false;
        navigator.push(Route::Login {});
    };

    rsx! {
        nav {
            class: "bg-gradient-to-r from-blue-600 to-blue-700 text-white px-6 py-4 shadow-lg",
            div {
                class: "max-w-6xl mx-auto flex justify-between items-center",
                Link {
                    to: Route::Home {},
                    class: "text-2xl font-bold hover:text-blue-100 transition-colors",
                    "📝 Todo & 💰 Tax App"
                }
                div {
                    class: "flex gap-4 items-center",
                    Link {
                        to: Route::Home {},
                        class: "hover:text-blue-100 transition-colors",
                        "Todos"
                    }
                    Link {
                        to: Route::Create {},
                        class: "bg-emerald-500 hover:bg-emerald-600 px-4 py-2 rounded-lg font-medium transition-colors",
                        "Add Todo"
                    }
                    button {
                        class: "bg-red-500 hover:bg-red-600 px-4 py-2 rounded-lg font-medium transition-colors",
                        onclick: handle_logout,
                        "Logout"
                    }
                }
            }
        }
        div {
            class: "min-h-screen bg-slate-50",
            Outlet::<Route> {}
        }
    }
}
