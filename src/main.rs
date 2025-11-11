use dioxus::prelude::*;
static CSS: Asset = asset!("assets/main.css");

fn main() {
    dioxus::launch(App);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[component]
fn App() -> Element {
    let mut result = use_signal(String::new);
    let mut show_result = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut show_error = use_signal(|| false);
    let mut num1 = use_signal(String::new);
    let mut num2 = use_signal(String::new);

    let mut result_handler = move || {
        if let (Ok(value1), Ok(value2)) = (num1().parse::<i32>(), num2().parse::<i32>()) {
            let answer = multiply(value1, value2).to_string();
            result.set(answer);
            show_result.set(true);
            show_error.set(false);
        } else {
            error.set(String::from("Error calculating result."));
            show_error.set(true);
            show_result.set(false);
        }
    };

    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "container",
            h1 { "Number Multiplier" }
            p { class: "subtitle", "Calculated in Rust. Very advanced mathematics" }

            div { class: "input-group",
                label { for: "num1", "First Number" }
                input {
                    oninput: move |evt| num1.set(evt.value()),
                    type: "number",
                    id: "num1",
                    placeholder: "Enter first number",
                    step: "any",
                }
            }

            div { class: "input-group",
                label { for: "num2", "Second Number" }
                input {
                    oninput: move |evt| num2.set(evt.value()),
                    type: "number",
                    id: "num2",
                    placeholder: "Enter second number",
                    step: "any",
                }
            }

            button {
                onclick: move |_| result_handler(),
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
