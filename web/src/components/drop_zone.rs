use dioxus::prelude::*;
use crate::components::{use_drag_drop_context, DraggableComponent, ComponentType};

#[component]
pub fn DropZone() -> Element {
    let drag_context = use_drag_drop_context();
    let drag_context_export_html = use_drag_drop_context();
    let drag_context_export_rsx = use_drag_drop_context();
    let drag_context_render = use_drag_drop_context();
    
    rsx! {
        div {
            id: "drop-zone",
            class: "flex-1 p-4 border-2 border-dashed border-gray-300 min-h-96 relative bg-gray-50 rounded-lg",
            ondragover: move |e| e.prevent_default(),
            ondrop: move |e| {
                e.prevent_default();
                if let Some(dragged_id) = drag_context.take_currently_dragging() {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(drop_zone_elem) = document.get_element_by_id("drop-zone")
                            {
                                let drop_zone_rect = drop_zone_elem.get_bounding_client_rect();
                                let point = e.client_coordinates();
                                let x = point.x - drop_zone_rect.left() - 10.0;
                                let y = point.y - drop_zone_rect.top() - 40.0;
                                drag_context
                                    .move_to_drop_zone(&dragged_id, (x.max(0.0), y.max(0.0)));
                            }
                        }
                    }
                }
            },
            div { class: "text-center text-gray-500 mb-4",
                h2 { class: "text-2xl font-semibold mb-2", "Canvas" }
                p { "Drag components from the left panel to build your layout" }
            }
            
            // Export buttons
            div { class: "absolute top-4 right-4 flex gap-2",
                button {
                    class: "px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600",
                    onclick: move |_| {
                        let html = drag_context_export_html.export_to_html();
                        // Log to console for now - in a real app you'd download or copy to clipboard
                        web_sys::console::log_1(&format!("HTML Export:\n{}", html).into());
                    },
                    "Export HTML"
                }
                button {
                    class: "px-3 py-1 bg-green-500 text-white rounded text-sm hover:bg-green-600",
                    onclick: move |_| {
                        let rsx = drag_context_export_rsx.export_to_rsx();
                        // Log to console for now - in a real app you'd download or copy to clipboard
                        web_sys::console::log_1(&format!("RSX Export:\n{}", rsx).into());
                    },
                    "Export RSX"
                }
            }
            
            // Render dropped components
            {
                drag_context_render
                    .get_components_in_drop_zone()
                    .into_iter()
                    .filter(|c| c.parent_id.is_none()) // Only render top-level components
                    .map(|component| {
                        let component_id = component.id.clone();
                        rsx! {
                            DroppedComponent { key: "{component_id}", component: component.clone() }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn DroppedComponent(component: DraggableComponent) -> Element {
    let drag_context = use_drag_drop_context();
    let drag_context_delete = use_drag_drop_context();
    let x = component.position.0;
    let y = component.position.1;
    let component_id = component.id.clone();
    let component_id_for_delete = component.id.clone();
    
    rsx! {
        div {
            class: "absolute bg-white border border-gray-300 rounded-lg p-3 shadow-sm hover:shadow-md transition-shadow cursor-move",
            style: "left: {x}px; top: {y}px;",
            draggable: "true",
            ondragstart: move |_e| {
                drag_context.set_currently_dragging(Some(component_id.clone()));
            },
            // Component content based on type
            {render_component_content(&component)}
            // Optional delete button
            button {
                class: "absolute -top-2 -right-2 w-6 h-6 bg-red-500 text-white rounded-full text-xs hover:bg-red-600",
                onclick: move |_| {
                    drag_context_delete.remove_component(&component_id_for_delete);
                },
                "×"
            }
        }
    }
}

fn render_component_content(component: &DraggableComponent) -> Element {
    match &component.component_type {
        ComponentType::OneColumn | ComponentType::TwoColumns | ComponentType::ThreeColumns | 
        ComponentType::FourColumns | ComponentType::FiveColumns => {
            let column_count = match component.component_type {
                ComponentType::OneColumn => 1,
                ComponentType::TwoColumns => 2,
                ComponentType::ThreeColumns => 3,
                ComponentType::FourColumns => 4,
                ComponentType::FiveColumns => 5,
                _ => 1,
            };
            
            let grid_class = match column_count {
                1 => "grid-cols-1",
                2 => "grid-cols-2",
                3 => "grid-cols-3",
                4 => "grid-cols-4",
                5 => "grid-cols-5",
                _ => "grid-cols-1",
            };
            
            rsx! {
                div { class: "min-w-96 p-2",
                    div { class: "grid {grid_class} gap-2 min-h-32",
                        {(0..column_count).map(|i| {
                            let column_id = format!("{}-col-{}", component.id, i);
                            rsx! {
                                ColumnDropZone { 
                                    key: "{column_id}",
                                    column_id: column_id,
                                    parent_component_id: component.id.clone(),
                                    column_index: i 
                                }
                            }
                        })}
                    }
                }
            }
        },
        ComponentType::Header => rsx! {
            div { class: "min-w-32",
                h1 { class: "text-xl font-bold text-gray-800", "{component.name}" }
            }
        },
        ComponentType::Hero => rsx! {
            div { class: "min-w-48 text-center p-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded",
                h2 { class: "text-lg font-bold", "{component.name}" }
                p { class: "text-sm", "This is a hero section" }
            }
        },
        ComponentType::Text => rsx! {
            div { class: "min-w-32",
                p { class: "text-gray-700", "{component.name}" }
            }
        },
        ComponentType::Image => rsx! {
            div { class: "min-w-32 h-24 bg-gray-200 rounded flex items-center justify-center",
                svg {
                    class: "w-8 h-8 text-gray-400",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "{component.component_type.icon()}",
                    }
                }
            }
        },
        ComponentType::Button => rsx! {
            button { class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                "{component.name}"
            }
        },
        ComponentType::Form => rsx! {
            div { class: "min-w-48 p-3 border border-gray-200 rounded",
                div { class: "mb-2",
                    label { class: "block text-sm font-medium", "Input Field" }
                    input {
                        class: "w-full px-2 py-1 border border-gray-300 rounded",
                        r#type: "text",
                    }
                }
                button { class: "px-3 py-1 bg-green-500 text-white rounded text-sm", "Submit" }
            }
        },
        ComponentType::Card => rsx! {
            div { class: "min-w-32 p-3 bg-white border border-gray-200 rounded-lg shadow-sm",
                h3 { class: "font-semibold text-gray-800", "{component.name}" }
                p { class: "text-sm text-gray-600", "Card content" }
            }
        },
        ComponentType::Footer => rsx! {
            div { class: "min-w-48 p-2 bg-gray-800 text-white text-center rounded",
                p { class: "text-sm", "{component.name}" }
            }
        },
        ComponentType::Custom(name) => rsx! {
            div { class: "min-w-32 p-2 border border-dashed border-gray-400 rounded",
                span { class: "text-sm text-gray-600", "{name}" }
            }
        },
    }
}

#[component]
pub fn ColumnDropZone(column_id: String, parent_component_id: String, column_index: usize) -> Element {
    let drag_context = use_drag_drop_context();
    let drag_context_render = use_drag_drop_context();
    let drag_context_check = use_drag_drop_context();
    
    rsx! {
        div {
            id: "{column_id}",
            class: "border-2 border-dashed border-gray-200 min-h-24 p-2 bg-gray-50 rounded",
            ondragover: move |e| e.prevent_default(),
            ondrop: move |e| {
                e.prevent_default();
                if let Some(dragged_id) = drag_context.take_currently_dragging() {
                    drag_context.add_to_column(&dragged_id, &parent_component_id, column_index);
                }
            },
            
            // Show column children
            {
                drag_context_render
                    .get_components_in_column(&parent_component_id)
                    .into_iter()
                    .filter(|c| {
                        // Filter components for this specific column based on position
                        let column_count = drag_context_render.get_column_count(&parent_component_id);
                        let column_width = 100.0 / column_count as f64;
                        let component_column = (c.position.0 / column_width) as usize;
                        component_column == column_index
                    })
                    .map(|component| {
                        let component_id = component.id.clone();
                        rsx! {
                            ColumnComponent { key: "{component_id}", component: component.clone() }
                        }
                    })
            }
            
            // Empty state text
            {
                let components_in_column = drag_context_check.get_components_in_column(&parent_component_id);
                if components_in_column.is_empty() {
                    rsx! {
                        div { class: "text-xs text-gray-400 text-center py-2",
                            "Drop components here"
                        }
                    }
                } else {
                    rsx! { }
                }
            }
        }
    }
}

#[component]
pub fn ColumnComponent(component: DraggableComponent) -> Element {
    let drag_context = use_drag_drop_context();
    let drag_context_delete = use_drag_drop_context();
    let component_id = component.id.clone();
    let component_id_for_delete = component.id.clone();
    
    rsx! {
        div {
            class: "bg-white border border-gray-300 rounded p-2 mb-2 shadow-sm hover:shadow-md transition-shadow cursor-move",
            draggable: "true",
            ondragstart: move |_e| {
                drag_context.set_currently_dragging(Some(component_id.clone()));
            },
            
            // Component content - simplified for column display
            {render_column_component_content(&component)}
            
            // Delete button
            button {
                class: "absolute -top-1 -right-1 w-5 h-5 bg-red-500 text-white rounded-full text-xs hover:bg-red-600",
                onclick: move |_| {
                    drag_context_delete.remove_component(&component_id_for_delete);
                },
                "×"
            }
        }
    }
}

fn render_column_component_content(component: &DraggableComponent) -> Element {
    match &component.component_type {
        ComponentType::Header => rsx! {
            h3 { class: "text-sm font-bold text-gray-800", "{component.name}" }
        },
        ComponentType::Hero => rsx! {
            div { class: "text-center p-2 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded text-xs",
                h4 { class: "font-bold", "{component.name}" }
            }
        },
        ComponentType::Text => rsx! {
            p { class: "text-xs text-gray-700", "{component.name}" }
        },
        ComponentType::Button => rsx! {
            button { class: "px-2 py-1 bg-blue-500 text-white rounded text-xs",
                "{component.name}"
            }
        },
        ComponentType::Card => rsx! {
            div { class: "p-2 bg-white border border-gray-200 rounded shadow-sm",
                h4 { class: "text-xs font-semibold text-gray-800", "{component.name}" }
            }
        },
        ComponentType::Footer => rsx! {
            div { class: "p-1 bg-gray-800 text-white text-center rounded text-xs",
                "{component.name}"
            }
        },
        _ => rsx! {
            div { class: "p-1 border border-gray-300 rounded text-xs text-gray-600",
                "{component.name}"
            }
        },
    }
}
