use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut logged_in = use_context::<Signal<bool>>();
    let navigator = use_navigator();

    let handle_login = move |_: Event<MouseData>| {
        if email() == "admin@gmail.com" && password() == "helloworld" {
            *logged_in.write() = true;
            *error.write() = String::new();
            navigator.push(Route::Home {});
        } else {
            *error.write() = "Invalid email or password".to_string();
        }
    };

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-blue-50 to-slate-100 flex items-center justify-center px-4",
            div {
                class: "w-full max-w-md bg-white rounded-xl shadow-lg border border-slate-200 p-8",
                div {
                    class: "mb-8 text-center",
                    h1 {
                        class: "text-3xl font-bold text-slate-900 mb-2",
                        "📝 Todo App"
                    }
                    p {
                        class: "text-slate-500",
                        "Login to manage your todos"
                    }
                }

                if !error().is_empty() {
                    div {
                        class: "mb-6 p-4 bg-red-50 border border-red-200 rounded-lg",
                        p {
                            class: "text-red-700 text-sm font-medium",
                            "{error}"
                        }
                    }
                }

                div {
                    class: "space-y-6",
                    div {
                        label {
                            class: "block text-sm font-semibold text-slate-700 mb-2",
                            r#for: "email",
                            "Email"
                        }
                        input {
                            class: "w-full px-4 py-3 border border-slate-300 rounded-lg bg-slate-50 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition text-slate-900",
                            id: "email",
                            r#type: "email",
                            placeholder: "admin@gmail.com",
                            value: "{email}",
                            oninput: move |e| *email.write() = e.value(),
                        }
                    }

                    div {
                        label {
                            class: "block text-sm font-semibold text-slate-700 mb-2",
                            r#for: "password",
                            "Password"
                        }
                        input {
                            class: "w-full px-4 py-3 border border-slate-300 rounded-lg bg-slate-50 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition text-slate-900",
                            id: "password",
                            r#type: "password",
                            placeholder: "Enter password",
                            value: "{password}",
                            oninput: move |e| *password.write() = e.value(),
                        }
                    }

                    button {
                        class: "w-full bg-blue-600 hover:bg-blue-700 disabled:bg-slate-300 disabled:cursor-not-allowed text-white px-6 py-3 rounded-lg font-semibold transition-colors",
                        onclick: handle_login,
                        disabled: email().is_empty() || password().is_empty(),
                        "Login"
                    }

                    div {
                        class: "pt-4 border-t border-slate-200",
                        p {
                            class: "text-slate-600 text-sm text-center",
                            "Demo credentials:"
                            br {}
                            "Email: admin@gmail.com"
                            br {}
                            "Password: helloworld"
                        }
                    }
                }
            }
        }
    }
}
