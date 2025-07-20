use dioxus::prelude::*;
use crate::components::{ComponentType, use_drag_drop_context};

#[component]
pub fn BlockItem(component_type: ComponentType) -> Element {
    let drag_context = use_drag_drop_context();
    let name = component_type.default_content();
    let icon = component_type.icon();
    
    rsx! {
        div {
            class: "bg-white p-4 rounded-lg border border-gray-200 hover:border-blue-400 cursor-move \
                   flex flex-col items-center text-center",
            draggable: "true",
            ondragstart: move |_e| {
                let component_id = drag_context.add_component(component_type.clone());
                drag_context.set_currently_dragging(Some(component_id));
            },
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
            h2 { class: "text-xl font-semibold mb-4", "Columns" }
            div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
                // Example blocks - these would be draggable in a real implementation
                BlockItem { component_type: ComponentType::OneColumn }
                BlockItem { component_type: ComponentType::TwoColumns }
                BlockItem { component_type: ComponentType::ThreeColumns }
                BlockItem { component_type: ComponentType::FourColumns }
                BlockItem { component_type: ComponentType::FiveColumns }
            }

            div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
                // Example blocks - these would be draggable in a real implementation
                BlockItem { component_type: ComponentType::Header }
                BlockItem { component_type: ComponentType::Hero }
                BlockItem { component_type: ComponentType::Text }
                BlockItem { component_type: ComponentType::Image }
                BlockItem { component_type: ComponentType::Button }
                BlockItem { component_type: ComponentType::Form }
            }
        }
    }
}
