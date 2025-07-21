use dioxus::prelude::*;
use crate::components::{ComponentType, use_drag_drop_context};

#[component]
pub fn BlockItem(component_type: ComponentType) -> Element {
    let drag_context = use_drag_drop_context();
    let name = component_type.default_content();
    let icon = component_type.icon();
    
    // Different styling for layout vs content components
    let (border_color, hover_color, icon_color) = match component_type {
        ComponentType::OneColumn | ComponentType::TwoColumns | ComponentType::ThreeColumns |
        ComponentType::FourColumns | ComponentType::FiveColumns => {
            ("border-blue-200", "hover:border-blue-400 hover:bg-blue-50", "text-blue-600")
        },
        _ => {
            ("border-green-200", "hover:border-green-400 hover:bg-green-50", "text-green-600")
        }
    };
    
    rsx! {
        div {
            class: "bg-white p-3 rounded-lg border {border_color} {hover_color} cursor-move \
                   flex flex-col items-center text-center transition-colors",
            draggable: "true",
            ondragstart: move |_e| {
                let component_id = drag_context.add_component(component_type.clone());
                drag_context.set_currently_dragging(Some(component_id));
            },
            svg {
                class: "w-6 h-6 mb-2 {icon_color}",
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
            span { class: "text-xs font-medium text-gray-700", "{name}" }
        }
    }
}

#[component]
pub fn BlocksPanel() -> Element {
    rsx! {
        div { class: "space-y-6",
            // Layout Components Section
            div {
                h2 { class: "text-lg font-semibold mb-3 text-blue-600 border-b border-blue-200 pb-2", "📐 Layout Components" }
                p { class: "text-sm text-gray-600 mb-4", "Drop these on the main canvas to create your page structure" }
                div { class: "grid grid-cols-1 gap-3",
                    BlockItem { component_type: ComponentType::OneColumn }
                    BlockItem { component_type: ComponentType::TwoColumns }
                    BlockItem { component_type: ComponentType::ThreeColumns }
                    BlockItem { component_type: ComponentType::FourColumns }
                    BlockItem { component_type: ComponentType::FiveColumns }
                }
            }

            // Content Components Section  
            div {
                h2 { class: "text-lg font-semibold mb-3 text-green-600 border-b border-green-200 pb-2", "🧩 Content Components" }
                p { class: "text-sm text-gray-600 mb-4", "Drop these into column areas to add content" }
                div { class: "grid grid-cols-2 gap-3",
                    BlockItem { component_type: ComponentType::Header }
                    BlockItem { component_type: ComponentType::Hero }
                    BlockItem { component_type: ComponentType::Text }
                    BlockItem { component_type: ComponentType::Image }
                    BlockItem { component_type: ComponentType::Button }
                    BlockItem { component_type: ComponentType::Form }
                    BlockItem { component_type: ComponentType::Card }
                    BlockItem { component_type: ComponentType::Footer }
                }
            }
        }
    }
}
