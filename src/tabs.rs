use dioxus::prelude::*;
static CSS: Asset = asset!("assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Home,
    Settings,
    Stats,
}

#[component]
fn App() -> Element {
    let mut active_tab = use_signal(|| Tab::Home);

    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "app",

            // -------- TABS ----------
            div { class: "tabs",
                button {
                    class: if active_tab() == Tab::Home { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(Tab::Home),
                    "Home"
                }

                button {
                    class: if active_tab() == Tab::Settings { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(Tab::Settings),
                    "Settings"
                }

                button {
                    class: if active_tab() == Tab::Stats { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(Tab::Stats),
                    "Stats"
                }
            }

            // -------- CONTENT ----------
            div { class: "content",
                match active_tab() {
                    Tab::Home => rsx!{ HomeView {} },
                    Tab::Settings => rsx!{ SettingsView {} },
                    Tab::Stats => rsx!{ StatsView {} },
                }
            }
        }
    }
}

#[component]
fn HomeView() -> Element {
    rsx! {
        h2 { "Home" }
        p { "This is the home tab. Put your dashboard stuff here." }
    }
}

#[component]
fn SettingsView() -> Element {
    rsx! {
        h2 { "Settings" }
        p { "These settings do absolutely nothing. Yet." }
    }
}

#[component]
fn StatsView() -> Element {
    rsx! {
        h2 { "Stats" }
        p { "Stats go here. (Charts, metrics, numbers, etc.)" }
    }
}
