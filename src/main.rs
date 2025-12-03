use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct ClientData {
    name: String,
    age: String,
    income: String,
    mortgage: String,
}

#[derive(Clone, PartialEq, Debug)]
struct ProcessingResult {
    client_data: ClientData,
    approved: bool,
    commitment: String,
    proof: String,
    signature: String,
}

#[derive(Clone, PartialEq, Debug)]
enum Screen {
    ClientInput,
    ServerProcessing,
    ObserverReview,
}

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::INFO).expect("logger failed to init");
    launch(app);
}

fn process_input(data: ClientData) -> ProcessingResult {
    let age_num: u32 = data.age.parse().unwrap_or(0);
    let income_num: f64 = data.income.parse().unwrap_or(0.0);
    let score = (income_num / 1000.0) + (age_num as f64 * 1.5);

    let approved = score > 100.0;
    let commitment = "COMMITMENT".to_string();
    let proof = "PROOF".to_string();
    let signature = "SIGNATURE".to_string();

    info!("Processing complete: score={}", score);

    ProcessingResult {
        client_data: data,
        approved,
        commitment,
        proof,
        signature,
    }
}

fn app() -> Element {
    let mut current_screen = use_signal(|| Screen::ClientInput);
    let mut client_data = use_signal(|| ClientData {
        name: String::new(),
        age: String::new(),
        income: String::new(),
        mortgage: String::new(),
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
                                mortgage:String::new()
                            });
                            result.set(None);
                            current_screen.set(Screen::ClientInput);
                        }
                    }
                }
            }
        }

        // Debug section
        div { class: "white-text",
            h1 {"Debug section"}
            h5 {"current_screen {current_screen:?}"}
            h5 {"client_data {client_data:?}"}
            h5 {"result {result:?}"}
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
        !data.name.is_empty()
            && !data.age.is_empty()
            && !data.income.is_empty()
            && !data.mortgage.is_empty()
    };

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

                    div { class: "form-group",
                        label { "Mortgage amount ($):" }
                        input {
                            class: "input",
                            r#type: "number",
                            value: "{client_data.read().mortgage}",
                            placeholder: "Enter the mortgage amount",
                            oninput: move |e| client_data.write().mortgage = e.value(),
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        disabled: !is_valid,
                        onclick: move |_| on_submit.call(()),
                        "Submit to Bank →"
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
    info!("active? {is_active}");
    let has_result = result.read().is_some();

    rsx! {
        div { class: "screen bank-screen",
            div { class: "screen-header",
                div { class: "step-indicator", "Step 2 of 3" }

                if is_active {
                    h1 { class: "screen-title", "Bank Processing" }
                    p { class: "screen-subtitle", "Analyzing submitted data..." }
                } else {
                    h1 { class: "screen-title-small", "Bank" }
                }
            }

            div { class: "processing-card",
                if is_active {
                    div { class: "processing-content",
                        if !has_result {
                            div { "<Some additional information...>" }
                            button {
                                class: "btn btn-primary process-button",
                                onclick: move |_| {
                                    let data = client_data.read().clone();
                                    let output = process_input(data);
                                    result.set(Some(output));
                                },
                                "Process input"
                            }
                        }
                        else {
                            div { class: "complete-state",
                                div { class: "checkmark", "✓" }
                                p { class: "complete-text", "Processing Complete!" }

                                if let Some(res) = result.read().as_ref() {
                                    div { class: "result-preview",
                                        p { "Name: {res.client_data.name}" }
                                        p { "Score: {res.approved}" }
                                        p { "Status: {res.proof}" }
                                    }
                                }

                                button {
                                    class: "btn btn-primary send-button",
                                    onclick: move |_| {
                                        info!("Sending to regulator...");
                                        on_complete.call(())
                                    },
                                    "Send to Regulator →"
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
        div { class: "screen regulator-screen",
            div { class: "screen-header",
                div { class: "step-indicator", "Step 3 of 3" }
                if is_active {
                    h1 { class: "screen-title", "Regulator Review" }
                    p { class: "screen-subtitle", "Verification of correctness" }
                } else {
                    h1 { class: "screen-title-small", "Regulator" }
                }
            }

            if let Some(res) = result.read().as_ref() {
                if is_active {
                    div { class: "review-card",
                        div { class: "review-section",
                            h3 { "Processing Results" }
                            div { class: "result-box",
                                div { class: "result-item",
                                    span { class: "result-label", "Approved:" }
                                    span { class: "result-value score", "{res.approved}" }
                                }
                                div { class: "result-item",
                                    span { class: "result-label", "Signature" }
                                    span { class: "result-value score", "{res.approved}" }
                                }
                                div { class: "result-item",
                                    span { class: "result-label", "Commitment" }
                                    span { class: "result-value score", "{res.approved}" }
                                }
                                div { class: "result-item",
                                    span { class: "result-label", "Proof" }
                                    span { class: "result-value score", "{res.approved}" }
                                }
                            }
                        }

                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                info!("Restarting workflow...");
                                on_restart.call(())
                            },
                            "Verify signature"
                        }

                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                info!("Restarting workflow...");
                                on_restart.call(())
                            },
                            "Verify proof"
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

    /* Bank Screen - Green/Teal Theme */
    .bank-screen {
        background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
    }

    /* Regulator Screen - Pink/Red Theme */
    .regulator-screen {
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

    .bank-screen .btn-primary {
        background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
        color: white;
    }

    .regulator-screen .btn-secondary {
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

    .regulator-notes {
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

    .regulator-notes:focus {
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

    .process-button {
        margin-top: 2rem;
    }

    .complete-state {
        animation: fadeIn 0.5s ease;
    }

    .processing-content .process-button {
        display: block;
        margin-top: 4rem;
    }

    .result-preview {
        margin-top: 1.5rem;
    }

    .send-button {
        margin-top: 1.5rem;
    }
"#;
