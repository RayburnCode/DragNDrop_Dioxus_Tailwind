use dioxus::prelude::*;
use crate::components::DropZone;

#[component]
pub fn DragDropDemo() -> Element {
    rsx! {
        div { class: "h-full flex flex-col",
            div { class: "flex-1 p-6",
                h1 { class: "text-3xl font-bold text-gray-800 mb-6", "Drag & Drop Builder" }
                p { class: "text-gray-600 mb-8", 
                    "Drag components from the left panel to build your layout. Use the blocks section in the left navigation."
                }
                DropZone {}
            }
        }
    }
}
