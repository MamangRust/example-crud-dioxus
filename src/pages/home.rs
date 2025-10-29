use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::Todo;

#[component]
pub fn Home() -> Element {
    let logged_in = use_context::<Signal<bool>>();
    let navigator = use_navigator();

    if !logged_in() {
        navigator.push(Route::Login {});
        return rsx! { div {} };
    }

    let mut todos = use_context::<Signal<Vec<Todo>>>();
    let mut delete_id = use_signal(|| Option::<usize>::None);

    let is_deleting = delete_id().is_some();
    let todo_to_delete = delete_id().and_then(|id| todos().iter().find(|t| t.id == id).cloned());
    let label = format!("{} todos", todos().len());

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
            div {
                class: "mb-8",
                h1 {
                    class: "text-4xl font-bold text-slate-900 mb-2",
                    "My Todos"
                }
                p {
                    class: "text-slate-500 text-lg",
                    "{label}"
                }
            }

            if todos().is_empty() {
                div {
                    class: "bg-white rounded-xl shadow-sm p-12 text-center",
                    p {
                        class: "text-slate-500 text-lg mb-6",
                        "No todos yet. Create one to get started!"
                    }
                    Link {
                        to: Route::Create {},
                        class: "inline-block bg-emerald-500 hover:bg-emerald-600 text-white px-6 py-3 rounded-lg font-medium transition-colors",
                        "Create First Todo"
                    }
                }
            } else {
                div {
                    class: "grid gap-4 md:gap-6",
                    for todo in todos() {
                        div {
                            class: "bg-white rounded-xl shadow-sm p-6 hover:shadow-md transition-all duration-200",
                            div {
                                class: "flex justify-between items-start gap-4 mb-4",
                                h2 {
                                    class: "text-xl font-bold text-slate-900 flex-1",
                                    "{todo.title}"
                                }
                                div {
                                    class: "flex gap-2 flex-shrink-0",
                                    Link {
                                        to: Route::Edit { id: todo.id },
                                        class: "inline-flex bg-blue-50 hover:bg-blue-100 text-blue-700 px-4 py-2 rounded-lg text-sm font-medium transition-colors",
                                        "Edit"
                                    }
                                    button {
                                        class: "bg-red-50 hover:bg-red-100 text-red-700 px-4 py-2 rounded-lg text-sm font-medium transition-colors",
                                        onclick: move |_| *delete_id.write() = Some(todo.id),
                                        "Delete"
                                    }
                                }
                            }
                            p {
                                class: "text-slate-600 leading-relaxed",
                                "{todo.description}"
                            }
                        }
                    }
                }
            }

            if is_deleting {
                div {
                    class: "fixed inset-0 bg-slate-950/10 flex items-center justify-center z-50",
                    onclick: move |_| *delete_id.write() = None,
                    div {
                        class: "bg-white rounded-xl shadow-2xl p-6 max-w-sm mx-4",
                        onclick: move |e: Event<MouseData>| e.stop_propagation(),
                        h2 {
                            class: "text-2xl font-bold text-slate-900 mb-4",
                            "Delete Todo?"
                        }
                        if let Some(todo) = todo_to_delete {
                            p {
                                class: "text-slate-600 mb-8 leading-relaxed",
                                "Are you sure you want to delete \"{todo.title}\"? This action cannot be undone."
                            }
                        }
                        div {
                            class: "flex gap-3 justify-end",
                            button {
                                class: "px-4 py-2 rounded-lg bg-slate-100 hover:bg-slate-200 text-slate-700 font-medium transition-colors",
                                onclick: move |_| *delete_id.write() = None,
                                "Cancel"
                            }
                            button {
                                class: "px-4 py-2 rounded-lg bg-red-500 hover:bg-red-600 text-white font-medium transition-colors",
                                onclick: move |_| {
                                    let delete_todo_id = *delete_id.read();
                                    if let Some(id) = delete_todo_id {
                                        todos.write().retain(|t| t.id != id);
                                        *delete_id.write() = None;
                                    }
                                },
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
