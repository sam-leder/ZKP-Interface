use dioxus::logger::tracing::info;
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // --- Three states ---
    let mut count = use_signal(|| 0);
    let mut name = use_signal(|| "Alice".to_string());
    let mut flag = use_signal(|| false);

    // --- An effect that logs when anything changes ---
    use_effect(move || {
        info!(
            "Effect ran: count={}, name={}, flag={}",
            count(),
            name(),
            flag()
        );
    });

    rsx! {
        div {
            h1 { "Three-State Example" }

            // --- Count state ---
            div {
                "Count: {count()} "
                button { onclick: move |_| count.set(count() + 1), "+" }
                button { onclick: move |_| count.set(count() - 1), "-" }
            }

            // --- Name state ---
            div {
                "Name: {name()} "
                button { onclick: move |_| name.set("Bob".into()), "Set Bob" }
                button { onclick: move |_| name.set("Carol".into()), "Set Carol" }
            }

            // --- Flag state ---
            div {
                "Flag: {flag()} "
                button { onclick: move |_| flag.set(!flag()), "Toggle" }
            }

            div{ my_comp {n: count()} }
        }
    }
}

#[component]
fn my_comp(n: i32) -> Element {
    let res = n + 1;
    rsx!(div {"hi {res}"})
}
