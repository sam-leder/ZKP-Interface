use dioxus::prelude::*;
// use std::time::Duration;

use dioxus::logger::tracing::{info, Level};

#[derive(Clone, PartialEq, Debug)]
struct ClientData {
    name: String,
    age: String,
    income: String,
}

#[derive(Clone, PartialEq, Debug)]
struct ProcessingResult {
    client_data: ClientData,
    score: f64,
    status: String,
}

#[derive(Clone, PartialEq, Debug)]
enum Screen {
    ClientInput,
    ServerProcessing,
    ObserverReview,
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger failed to init");
    launch(app);
}

fn app() -> Element {
    let mut current_screen = use_signal(|| Screen::ClientInput);
    let mut client_data = use_signal(|| ClientData {
        name: String::new(),
        age: String::new(),
        income: String::new(),
    });
    let mut result = use_signal(|| None::<ProcessingResult>);

    // Debug: Log screen changes
    use_effect(move || {
        let screen_name = match current_screen() {
            Screen::ClientInput => "ClientInput",
            Screen::ServerProcessing => "ServerProcessing",
            Screen::ObserverReview => "ObserverReview",
        };
        info!("Current screen: {}", screen_name);
    });

    rsx! {
        style { {CSS} }
        div { class: "white-text",
            h1 {"Debug section"}
            h5 {"current_screen {current_screen:?}"}
            h5 {"client_data {client_data:?}"}
            h5 {"result {result:?}"}
        }
        div { class: "app-container",
            div { class: "screens-wrapper",
                // Screen 1
                div {
                    class: if current_screen() == Screen::ClientInput { "screen-panel active" } else { "screen-panel inactive" },
                    ClientInputScreen {
                        client_data,
                        is_active: current_screen() == Screen::ClientInput,
                        on_submit: move |_| {
                            current_screen.set(Screen::ServerProcessing);
                        }
                    }
                }

                // Screen 2
                div {
                    class: if current_screen() == Screen::ServerProcessing { "screen-panel active" } else { "screen-panel inactive" },
                    ServerProcessingScreen {
                        client_data,
                        result,
                        is_active: current_screen() == Screen::ServerProcessing,
                        on_complete: move |_| {
                            current_screen.set(Screen::ObserverReview);
                        }
                    }
                }

                // Screen 3
                div {
                    class: if current_screen() == Screen::ObserverReview { "screen-panel active" } else { "screen-panel inactive" },
                    ObserverReviewScreen {
                        result,
                        is_active: current_screen() == Screen::ObserverReview,
                        on_restart: move |_| {
                            client_data.set(ClientData {
                                name: String::new(),
                                age: String::new(),
                                income: String::new(),
                            });
                            result.set(None);
                            current_screen.set(Screen::ClientInput);
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ClientInputScreen(
    client_data: Signal<ClientData>,
    is_active: bool,
    on_submit: EventHandler<()>,
) -> Element {
    let is_valid = {
        let data = client_data.read();
        !data.name.is_empty() && !data.age.is_empty() && !data.income.is_empty()
    };

    // Debug logging
    use_effect(move || {
        if is_active {
            info!("Client screen is now active");
        }
    });

    rsx! {
        div { class: "screen client-screen",
            div { class: "screen-header",
                div { class: "step-indicator", "Step 1 of 3" }
                if is_active {
                    h1 { class: "screen-title", "Client Information" }
                    p { class: "screen-subtitle", "Please enter your details" }
                } else {
                    h1 { class: "screen-title-small", "Client" }
                }
            }

            div { class: "form-card",
                if is_active {
                    div { class: "form-group",
                        label { "Name:" }
                        input {
                            class: "input",
                            r#type: "text",
                            value: "{client_data.read().name}",
                            placeholder: "Enter your name",
                            oninput: move |e| client_data.write().name = e.value(),
                        }
                    }

                    div { class: "form-group",
                        label { "Age:" }
                        input {
                            class: "input",
                            r#type: "number",
                            value: "{client_data.read().age}",
                            placeholder: "Enter your age",
                            oninput: move |e| client_data.write().age = e.value(),
                        }
                    }

                    div { class: "form-group",
                        label { "Annual Income ($):" }
                        input {
                            class: "input",
                            r#type: "number",
                            value: "{client_data.read().income}",
                            placeholder: "Enter your income",
                            oninput: move |e| client_data.write().income = e.value(),
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        disabled: !is_valid,
                        onclick: move |_| on_submit.call(()),
                        "Submit to Server →"
                    }
                } else {
                    p { class: "inactive-message-vertical", "Waiting..." }
                }
            }
        }
    }
}

#[component]
fn ServerProcessingScreen(
    client_data: Signal<ClientData>,
    mut result: Signal<Option<ProcessingResult>>,
    is_active: bool,
    on_complete: EventHandler<()>,
) -> Element {
    let mut processing_started = use_signal(|| false);
    let mut processing_complete = use_signal(|| false);

    let mut start_processing = move || {
        if !processing_started() && is_active {
            processing_started.set(true);
            processing_complete.set(false); // Reset the flag

            let data = client_data.read().clone();

            // Debug log
            info!("Starting processing for: {}", data.name);

            // spawn(async move {
            // async_std::task::sleep(Duration::from_secs(2)).await;

            let age_num: u32 = data.age.parse().unwrap_or(0);
            let income_num: f64 = data.income.parse().unwrap_or(0.0);
            let score = (income_num / 1000.0) + (age_num as f64 * 1.5);

            let status = if score > 100.0 {
                "Approved - High Score"
            } else if score > 50.0 {
                "Approved - Moderate Score"
            } else {
                "Under Review"
            }
            .to_string();

            // Debug log
            info!("Processing complete! Score: {score}");

            result.set(Some(ProcessingResult {
                client_data: data,
                score,
                status,
            }));

            processing_complete.set(true);
            // });
        }
    };

    // Reset processing state when screen becomes inactive
    use_effect(move || {
        if !is_active {
            processing_started.set(false);
            processing_complete.set(false);
        }
    });

    use_effect(move || {
        if is_active {
            start_processing();
        }
    });

    rsx! {
        div { class: "screen server-screen",
            div { class: "screen-header",
                div { class: "step-indicator", "Step 2 of 3" }
                if is_active {
                    h1 { class: "screen-title", "Server Processing" }
                    p { class: "screen-subtitle", "Analyzing submitted data..." }
                } else {
                    h1 { class: "screen-title-small", "Server" }
                }
            }

            div { class: "processing-card",
                if is_active {
                    div { class: "processing-content",
                        if !processing_complete() {
                            div { class: "loading-state",
                                div { class: "spinner" }
                                p { class: "processing-text", "Processing data..." }
                                div { class: "progress-bar",
                                    div { class: "progress-fill" }
                                }
                            }
                        } else {
                            div { class: "complete-state",
                                div { class: "checkmark", "✓" }
                                p { class: "complete-text", "Processing Complete!" }

                                if let Some(res) = result.read().as_ref() {
                                    div { class: "result-preview",
                                        p { "Name: {res.client_data.name}" }
                                        p { "Score: {res.score:.2}" }
                                        p { "Status: {res.status}" }
                                    }
                                }

                                button {
                                    class: "btn btn-primary",
                                    onclick: move |_| {
                                        info!("Sending to observer...");
                                        on_complete.call(())
                                    },
                                    "Send to Observer →"
                                }
                            }
                        }
                    }
                } else {
                    p { class: "inactive-message-vertical", "Awaiting data" }
                }
            }
        }
    }
}

#[component]
fn ObserverReviewScreen(
    result: Signal<Option<ProcessingResult>>,
    is_active: bool,
    on_restart: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "screen observer-screen",
            div { class: "screen-header",
                div { class: "step-indicator", "Step 3 of 3" }
                if is_active {
                    h1 { class: "screen-title", "Observer Review" }
                    p { class: "screen-subtitle", "Third-party verification and review" }
                } else {
                    h1 { class: "screen-title-small", "Observer" }
                }
            }

            if let Some(res) = result.read().as_ref() {
                if is_active {
                    div { class: "review-card",
                        div { class: "review-section",
                            h3 { "Client Information" }
                            div { class: "info-grid",
                                div { class: "info-item",
                                    span { class: "info-label", "Name:" }
                                    span { class: "info-value", "{res.client_data.name}" }
                                }
                                div { class: "info-item",
                                    span { class: "info-label", "Age:" }
                                    span { class: "info-value", "{res.client_data.age}" }
                                }
                                div { class: "info-item",
                                    span { class: "info-label", "Income:" }
                                    span { class: "info-value", "${res.client_data.income}" }
                                }
                            }
                        }

                        div { class: "review-section",
                            h3 { "Processing Results" }
                            div { class: "result-box",
                                div { class: "result-item",
                                    span { class: "result-label", "Calculated Score:" }
                                    span { class: "result-value score", "{res.score:.2}" }
                                }
                                div { class: "result-item",
                                    span { class: "result-label", "Status:" }
                                    span {
                                        class: if res.status.contains("High") { "result-value status-high" }
                                            else {
                                                if res.status.contains("Moderate") { "result-value status-moderate" }
                                                    else { "result-value status-review" }
                                                },
                                        "{res.status}"
                                    }
                                }
                            }
                        }

                        div { class: "review-section",
                            h3 { "Observer Notes" }
                            textarea {
                                class: "observer-notes",
                                placeholder: "Add your observations here...",
                                rows: "4"
                            }
                        }

                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                info!("Restarting workflow...");
                                on_restart.call(())
                            },
                            "← Start New Review"
                        }
                    }
                } else {
                    div { class: "review-card",
                        p { class: "inactive-message-vertical", "Review pending" }
                    }
                }
            } else {
                div { class: "review-card",
                    p { class: "inactive-message-vertical", "Awaiting results" }
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
        background: #1a1a2e;
        min-height: 100vh;
        overflow-x: hidden;
    }

    .app-container {
        min-height: 100vh;
        padding: 20px;
    }

    .screens-wrapper {
        display: flex;
        gap: 10px;
        min-height: calc(100vh - 40px);
        transition: all 0.5s ease;
    }

    .screen-panel {
        transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .screen-panel.inactive {
        width: 10%;
        opacity: 0.6;
        filter: grayscale(0.3);
    }

    .screen-panel.inactive:hover {
        opacity: 0.8;
        filter: grayscale(0);
    }

    .screen-panel.active {
        width: 80%;
        opacity: 1;
    }

    .screen {
        flex: 1;
        display: flex;
        flex-direction: column;
        border-radius: 20px;
        padding: 2rem;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
        transition: all 0.5s ease;
    }

    /* Client Screen - Blue/Purple Theme */
    .client-screen {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    }

    /* Server Screen - Green/Teal Theme */
    .server-screen {
        background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
    }

    /* Observer Screen - Pink/Red Theme */
    .observer-screen {
        background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
    }

    .screen-header {
        text-align: center;
        margin-bottom: 2rem;
        transition: all 0.3s ease;
    }

    .inactive .screen-header {
        margin-bottom: 1rem;
    }

    .step-indicator {
        display: inline-block;
        background: rgba(255, 255, 255, 0.2);
        color: white;
        padding: 0.5rem 1rem;
        border-radius: 20px;
        font-size: 0.9rem;
        font-weight: 600;
        margin-bottom: 1rem;
    }

    .inactive .step-indicator {
        font-size: 0.7rem;
        padding: 0.3rem 0.6rem;
    }

    .screen-title {
        color: white;
        font-size: 2.5rem;
        margin-bottom: 0.5rem;
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
    }

    .screen-title-small {
        color: white;
        font-size: 1.2rem;
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.2);
        writing-mode: vertical-rl;
        text-orientation: mixed;
        margin: 0 auto;
    }

    .screen-subtitle {
        color: rgba(255, 255, 255, 0.9);
        font-size: 1.1rem;
    }

    .form-card, .processing-card, .review-card {
        background: white;
        border-radius: 20px;
        padding: 2.5rem;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
        flex: 1;
    }

    .inactive .form-card,
    .inactive .processing-card,
    .inactive .review-card {
        padding: 1rem;
    }

    .inactive-message {
        text-align: center;
        color: #718096;
        font-style: italic;
        padding: 2rem;
    }

    .inactive-message-vertical {
        text-align: center;
        color: #000000;
        font-style: italic;
        writing-mode: vertical-rl;
        text-orientation: mixed;
        margin: 0 auto;
        padding: 1rem 0;
        font-size: 0.9rem;
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

    .btn {
        width: 100%;
        padding: 1rem;
        border: none;
        border-radius: 10px;
        font-size: 1.1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;
        margin-top: 1rem;
    }

    .client-screen .btn-primary {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
    }

    .server-screen .btn-primary {
        background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
        color: white;
    }

    .observer-screen .btn-secondary {
        background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        color: white;
    }

    .btn-primary:hover:not(:disabled),
    .btn-secondary:hover {
        transform: translateY(-2px);
        box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
    }

    .btn-primary:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .processing-content {
        text-align: center;
        min-height: 300px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .loading-state {
        width: 100%;
    }

    .spinner {
        width: 80px;
        height: 80px;
        border: 6px solid #f3f4f6;
        border-top: 6px solid #11998e;
        border-radius: 50%;
        margin: 0 auto 2rem;
        animation: spin 1s linear infinite;
    }

    .processing-text {
        font-size: 1.3rem;
        color: #4a5568;
        font-weight: 600;
        margin-bottom: 2rem;
    }

    .progress-bar {
        width: 100%;
        height: 8px;
        background: #e2e8f0;
        border-radius: 10px;
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #11998e 0%, #38ef7d 100%);
        animation: progress 2s ease-in-out;
        width: 100%;
    }

    .complete-state {
        width: 100%;
    }

    .checkmark {
        width: 80px;
        height: 80px;
        background: #10b981;
        color: white;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 3rem;
        margin: 0 auto 1.5rem;
        animation: scaleIn 0.5s ease;
    }

    .complete-text {
        font-size: 1.5rem;
        color: #10b981;
        font-weight: 600;
        margin-bottom: 2rem;
    }

    .result-preview {
        background: #f7fafc;
        padding: 1.5rem;
        border-radius: 10px;
        margin-bottom: 1.5rem;
        text-align: left;
    }

    .result-preview p {
        color: #4a5568;
        margin-bottom: 0.5rem;
        font-size: 1rem;
    }

    .review-section {
        margin-bottom: 2rem;
    }

    .review-section:last-child {
        margin-bottom: 0;
    }

    .review-section h3 {
        color: #2d3748;
        margin-bottom: 1rem;
        font-size: 1.3rem;
    }

    .info-grid {
        display: grid;
        gap: 1rem;
    }

    .info-item {
        display: flex;
        justify-content: space-between;
        padding: 0.75rem;
        background: #f7fafc;
        border-radius: 8px;
    }

    .info-label {
        color: #718096;
        font-weight: 600;
    }

    .info-value {
        color: #2d3748;
        font-weight: 500;
    }

    .result-box {
        background: linear-gradient(135deg, #f093fb15 0%, #f5576c15 100%);
        padding: 1.5rem;
        border-radius: 10px;
        border: 2px solid #f093fb30;
    }

    .result-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .result-item:last-child {
        margin-bottom: 0;
    }

    .result-label {
        color: #4a5568;
        font-weight: 600;
        font-size: 1.1rem;
    }

    .result-value {
        font-weight: 700;
        font-size: 1.3rem;
    }

    .score {
        color: #f5576c;
    }

    .status-high {
        color: #10b981;
    }

    .status-moderate {
        color: #f59e0b;
    }

    .status-review {
        color: #ef4444;
    }

    .observer-notes {
        width: 100%;
        padding: 1rem;
        border: 2px solid #e2e8f0;
        border-radius: 10px;
        font-size: 1rem;
        font-family: inherit;
        resize: vertical;
        outline: none;
        transition: border-color 0.3s ease;
    }

    .observer-notes:focus {
        border-color: #f5576c;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    @keyframes progress {
        from {
            width: 0%;
        }
        to {
            width: 100%;
        }
    }

    @keyframes scaleIn {
        from {
            transform: scale(0);
        }
        to {
            transform: scale(1);
        }
    }

    .white-text {
        color: #ffffff;
    }
"#;
