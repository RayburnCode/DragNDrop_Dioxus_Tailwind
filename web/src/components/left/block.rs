use dioxus::prelude::*;

#[component]
pub fn BlockItem(name: String, icon: String) -> Element {
    rsx! {
        div { class: "bg-white p-4 rounded-lg border border-gray-200 hover:border-blue-400 cursor-move \
                   flex flex-col items-center text-center",
            svg {
                class: "w-8 h-8 mb-2 text-gray-600",
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
            span { class: "text-sm font-medium", "{name}" }
        }
    }
}


#[component]
// Placeholder components for each section
pub fn BlocksPanel() -> Element {
    rsx! {
        div {
            h2 { class: "text-xl font-semibold mb-4", "Building Blocks" }
            div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
                // Example blocks - these would be draggable in a real implementation
                BlockItem { name: "Header", icon: "M4 6h16M4 12h16M4 18h16" }
                BlockItem {
                    name: "Hero",
                    icon: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
                }
                BlockItem {
                    name: "Text",
                    icon: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                }
                BlockItem {
                    name: "Image",
                    icon: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
                }
                BlockItem { name: "Button", icon: "M5 12h14M12 5l7 7-7 7" }
                BlockItem {
                    name: "Form",
                    icon: "M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1",
                }
            }
        }
    }
}
