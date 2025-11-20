use dioxus::prelude::*;
use std::time::Duration;

#[derive(Clone, PartialEq)]
struct FormData {
    name: String,
    age: String,
    income: String,
}

#[derive(Clone, PartialEq)]
enum AppState {
    Form,
    Loading,
    Result(String),
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut form_data = use_signal(|| FormData {
        name: String::new(),
        age: String::new(),
        income: String::new(),
    });
    let mut app_state = use_signal(|| AppState::Form);

    let handle_submit = move |_| {
        let name = form_data.read().name.clone();
        let age = form_data.read().age.clone();
        let income = form_data.read().income.clone();

        // Validate inputs
        if name.is_empty() || age.is_empty() || income.is_empty() {
            return;
        }

        app_state.set(AppState::Loading);

        spawn(async move {
            // Simulate calculation with sleep
            async_std::task::sleep(Duration::from_secs(2)).await;

            // Simple processing
            // let age_num: u32 = age.parse().unwrap_or(0);
            let income_num: f64 = income.parse().unwrap_or(0.0);
            let score = income_num / 1000.0;
            let advice = if score > 50.0 {
                "qualify".to_string()
            } else {
                "do not qualify".to_string()
            };

            let result = format!(
                "Based on our model, you {advice} for a mortgage. You can verify the proof below (...)"
            );

            app_state.set(AppState::Result(result));
        });
    };

    let close_modal = move |_| {
        app_state.set(AppState::Form);
    };

    rsx! {
        style { {CSS} }
        div { class: "container",
            h1 { class: "title", "Mortgage application" }

            div { class: "form-card",
                p { class: "subtitle",
                    "Submit your information to find out if you qualify for a mortgage. "
                    "We also generate a Zero-knowledge Proof to prove that we followed an approved approach to determine the result of your application"
                }

                div { class: "form-group",
                    label { "Name:" }
                    input {
                        class: "input",
                        r#type: "text",
                        value: "{form_data.read().name}",
                        placeholder: "Enter your name",
                        oninput: move |e| form_data.write().name = e.value(),
                    }
                }

                div { class: "form-group",
                    label { "Age:" }
                    input {
                        class: "input",
                        r#type: "number",
                        value: "{form_data.read().age}",
                        placeholder: "Enter your age",
                        oninput: move |e| form_data.write().age = e.value(),
                    }
                }

                div { class: "form-group",
                    label { "Annual Income:" }
                    input {
                        class: "input",
                        r#type: "number",
                        value: "{form_data.read().income}",
                        placeholder: "Enter your income",
                        oninput: move |e| form_data.write().income = e.value(),
                    }
                }

                button {
                    class: "submit-btn",
                    onclick: handle_submit,
                    "Calculate"
                }
            }

            // Modal overlay
            if *app_state.read() != AppState::Form {
                div { class: "modal-overlay",
                    div { class: "modal",
                        match app_state.read().clone() {
                            AppState::Loading => rsx! {
                                div { class: "loading-container",
                                    div { class: "spinner" }
                                    p { class: "loading-text", "Generating Zero-knowledge proof..." }
                                }
                            },
                            AppState::Result(ref result) => rsx! {
                                div { class: "result-container",
                                    h2 { "Processing Complete" }
                                    p { class: "result-text", "{result}" }
                                    button {
                                        class: "close-btn",
                                        onclick: close_modal,
                                        "Close"
                                    }
                                }
                            },
                            _ => rsx!(div { p {"Something went wrong."} })
                        }
                    }
                }
            }
        }
    }
}

const CSS: &str = r#"
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    body {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        background: linear-gradient(135deg, #66bbff 0%, #884422 100%);
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .container {
        width: 100%;
        max-width: 500px;
        padding: 20px;
    }

    .title {
        color: white;
        text-align: center;
        font-size: 2.5rem;
        margin-bottom: 2rem;
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
    }

    .form-card {
        background: white;
        border-radius: 20px;
        padding: 2.5rem;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
    }

    .form-group {
        margin-bottom: 1.5rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        color: #4a5568;
        font-weight: 600;
        font-size: 0.95rem;
    }

    .subtitle {
        color: #666;
        text-align: center;
        margin-bottom: 30px;
        font-size: 14px;
    }

    .input {
        width: 100%;
        padding: 0.75rem 1rem;
        border: 2px solid #e2e8f0;
        border-radius: 10px;
        font-size: 1rem;
        transition: all 0.3s ease;
        outline: none;
    }

    .input:focus {
        border-color: #667eea;
        box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
    }

    .submit-btn {
        width: 100%;
        padding: 1rem;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        border: none;
        border-radius: 10px;
        font-size: 1.1rem;
        font-weight: 600;
        cursor: pointer;
        transition: transform 0.2s ease, box-shadow 0.2s ease;
        margin-top: 1rem;
    }

    .submit-btn:hover {
        transform: translateY(-2px);
        box-shadow: 0 10px 20px rgba(102, 126, 234, 0.3);
    }

    .submit-btn:active {
        transform: translateY(0);
    }

    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        animation: fadeIn 0.3s ease;
    }

    .modal {
        background: white;
        border-radius: 20px;
        padding: 3rem;
        max-width: 500px;
        width: 90%;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
        animation: slideUp 0.3s ease;
    }

    .loading-container {
        text-align: center;
    }

    .spinner {
        width: 60px;
        height: 60px;
        border: 5px solid #f3f4f6;
        border-top: 5px solid #667eea;
        border-radius: 50%;
        margin: 0 auto 1.5rem;
        animation: spin 1s linear infinite;
    }

    .loading-text {
        font-size: 1.5rem;
        color: #4a5568;
        font-weight: 600;
    }

    .result-container {
        text-align: center;
    }

    .result-container h2 {
        color: #667eea;
        margin-bottom: 1.5rem;
        font-size: 2rem;
    }

    .result-text {
        font-size: 1.1rem;
        color: #4a5568;
        line-height: 1.6;
        margin-bottom: 2rem;
    }

    .close-btn {
        padding: 0.75rem 2rem;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        border: none;
        border-radius: 10px;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: transform 0.2s ease;
    }

    .close-btn:hover {
        transform: translateY(-2px);
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    @keyframes slideUp {
        from {
            transform: translateY(50px);
            opacity: 0;
        }
        to {
            transform: translateY(0);
            opacity: 1;
        }
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
"#;
