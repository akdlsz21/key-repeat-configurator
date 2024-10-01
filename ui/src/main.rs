#![allow(non_snake_case)]

use dioxus::{desktop::LogicalSize, prelude::*};
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("#Key Repeat Configurator")
        .with_inner_size(LogicalSize::new(180, 120));

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="/assets/tailwind.css">"#.to_string())
        .with_window(window);

    LaunchBuilder::desktop().with_cfg(cfg).launch(App)
}

// fn get_default_rate() -> u32 {}

#[component]
fn App() -> Element {
    // Build cool things
    // OS'es default value to init use_signal.
    let mut input_delay = use_signal(|| 0);
    let mut repeat_rate = use_signal(|| 0);

    rsx! {
        section { class: "gap-4 w-screen text-lg font-bold text-gray-800 h-screen flex flex-col justify-center items-center bg-gray-100 p-4",
            div { class: "flex flex-row justify-between items-center w-full",
                p { class: "mr-2 border-1", "Input Delay " }
                p {
                    input {
                        class: "mr-4",
                        r#type: "range",
                        min: 1,
                        max: 400,
                        value: "{input_delay}",
                        oninput: move |event| { input_delay.set(event.value().parse().unwrap_or(0)) }
                    }
                    "{input_delay}"
                }
            }
            div { class: "flex flex-row justify-between items-center w-full",
                p { class: "mr-2 border-1", "Input Delay " }
                p {
                    input {
                        class: "mr-4",
                        r#type: "range",
                        min: 1,
                        max: 400,
                        oninput: move |event| { repeat_rate.set(event.value().parse().unwrap_or(0)) }
                    }
                    "{repeat_rate}"
                }
            }
        }
    }
}
