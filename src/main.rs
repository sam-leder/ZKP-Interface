use dioxus::prelude::*;
static CSS: Asset = asset!("assets/main.css");

fn main() {
    dioxus::launch(App);
}

fn result_handler(
    mut result: Signal<String>,
    mut show_result: Signal<bool>,
    error: Signal<String>,
    show_error: Signal<bool>,
    input1: String,
    input2: String,
) {
    if let (Ok(num1), Ok(num2)) = (input1.parse::<i32>(), input2.parse::<i32>()) {
        let answer = (num1 * num2).to_string();
        result.set(answer);
        show_result.set(true);
    } else {
        error_handler(error, show_error);
    }
}

fn error_handler(mut error: Signal<String>, mut show_error: Signal<bool>) {
    error.set(String::from("Error calculating result."));
    show_error.set(true);
}

#[component]
fn App() -> Element {
    let result = use_signal(|| String::from(""));
    let show_result = use_signal(|| false);
    let error: Signal<String> = use_signal(|| String::from(""));
    let show_error = use_signal(|| false);
    let mut num1: Signal<String> = use_signal(|| String::from(""));
    let mut num2: Signal<String> = use_signal(|| String::from(""));

    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "container",
            h1 { "Number Multiplier" }
            p { class: "subtitle", "Calculated in Rust. Very advanced mathematics" }

            div { class: "input-group",
                label { r#for: "num1", "First Number" }
                input {
                    oninput: move |evt| {
                        num1.set(evt.value().clone());
                    },
                    r#type: "number",
                    id: "num1",
                    placeholder: "Enter first number",
                    step: "any",
                }
            }

            div { class: "input-group",
                label { r#for: "num2", "Second Number" }
                input {
                    oninput: move |evt| {
                        num2.set(evt.value().clone());
                    },
                    r#type: "number",
                    id: "num2",
                    placeholder: "Enter second number",
                    step: "any",
                }
            }

            button {
                onclick: move |_| { result_handler(result, show_result, error, show_error, num1(), num2()) },
                id: "calculate",
                "Calculate Product"
            }

            div { class: if show_error() { "error show" } else { "error" }, id: "error", "{error}" }

            div {
                class: if show_result() { "result show" } else { "result" },
                id: "result",
                div { class: "result-label", "Result" }
                div { class: "result-value", id: "resultValue", "{result}" }
            }
        }
    }
}
