use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::{Todo, get_next_id};

#[component]
pub fn Create() -> Element {
    let logged_in = use_context::<Signal<bool>>();
    let navigator = use_navigator();

    if !logged_in() {
        navigator.push(Route::Login {});
        return rsx! { div {} };
    }

    let mut todos = use_context::<Signal<Vec<Todo>>>();
    let mut title = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());

    let handle_submit = move |_: Event<MouseData>| {
        if !title().is_empty() && !description().is_empty() {
            let new_todo = Todo {
                id: get_next_id(),
                title: title().clone(),
                description: description().clone(),
            };
            todos.write().push(new_todo);
            navigator.push(Route::Home {});
        }
    };

    rsx! {
        div {
            class: "max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
            div {
                class: "bg-white rounded-xl shadow-sm border border-slate-200 p-8",
                h1 {
                    class: "text-3xl font-bold text-slate-900 mb-8",
                    "Create New Todo"
                }
                div {
                    class: "space-y-6",
                    div {
                        label {
                            class: "block text-sm font-semibold text-slate-700 mb-2",
                            r#for: "title",
                            "Title"
                        }
                        input {
                            class: "w-full px-4 py-2 border border-slate-300 rounded-lg bg-slate-50 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition text-slate-900",
                            id: "title",
                            placeholder: "Enter todo title",
                            value: "{title}",
                            oninput: move |e| *title.write() = e.value(),
                        }
                    }
                    div {
                        label {
                            class: "block text-sm font-semibold text-slate-700 mb-2",
                            r#for: "description",
                            "Description"
                        }
                        textarea {
                            class: "w-full px-4 py-2 border border-slate-300 rounded-lg bg-slate-50 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none transition text-slate-900",
                            id: "description",
                            placeholder: "Enter todo description",
                            rows: "4",
                            value: "{description}",
                            oninput: move |e| *description.write() = e.value(),
                        }
                    }
                    div {
                        class: "flex gap-3 pt-6",
                        button {
                            class: "flex-1 bg-emerald-500 hover:bg-emerald-600 disabled:bg-slate-300 disabled:cursor-not-allowed text-white px-6 py-3 rounded-lg font-semibold transition-colors",
                            onclick: handle_submit,
                            disabled: title().is_empty() || description().is_empty(),
                            "Create Todo"
                        }
                        Link {
                            to: Route::Home {},
                            class: "flex-1 bg-slate-100 hover:bg-slate-200 text-slate-700 px-6 py-3 rounded-lg font-semibold text-center transition-colors",
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
