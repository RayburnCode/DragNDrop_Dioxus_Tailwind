use dioxus::prelude::*;

#[component]
pub fn IconCard(icon: String, title: String, description: String) -> Element {
    rsx! {
        div { class: "w-48 h-48 p-4 bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow \
                   flex flex-col items-center justify-center text-center border border-gray-100 \
                   hover:border-blue-200 cursor-pointer",
            // Icon container
            div { class: "w-16 h-16 mb-4 rounded-full bg-blue-50 flex items-center justify-center \
                       text-blue-600",
                svg {
                    class: "w-8 h-8",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "{icon}",
                    }
                }
            }
            // Text content
            h3 { class: "text-lg font-semibold text-gray-800 mb-2", "{title}" }
            p { class: "text-sm text-gray-500", "{description}" }
        }
    }
}

// Example usage:
#[component]
pub fn CardGrid() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 p-6",
            IconCard {
                icon: "M13 10V3L4 14h7v7l9-11h-7z".to_string(),
                title: "Fast Performance".to_string(),
                description: "Optimized for speed and efficiency".to_string(),
            }
            IconCard {
                icon: "M12 15l8-8m0 0h-8m8 0v8".to_string(),
                title: "Easy Integration".to_string(),
                description: "Works seamlessly with your stack".to_string(),
            }
            IconCard {
                icon: "M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"
                    .to_string(),
                title: "Cloud Ready".to_string(),
                description: "Deploy anywhere with one click".to_string(),
            }
        }
    }
}