use dioxus::prelude::*;
use chrono::Local;
use chrono::Datelike;

#[component]
pub fn Footer(children: Element) -> Element {
rsx! {

    div { class: "sm:flex sm:items-center sm:justify-between px-4 py-6 text-CustomAccent bg-CustomNav shadow-md",
        span { class: "text-sm text-CustomAccent sm:text-center ",
            "© {chrono::Local::now().year()}"
            a { class: "hover:underline", href: "#", " DragNDrop" }
            ". All Rights Reserved. "
        }
    }
}
    
}