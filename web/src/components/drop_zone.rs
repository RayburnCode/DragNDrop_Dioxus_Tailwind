use dioxus::prelude::*;
use crate::components::{use_drag_drop_context, DraggableComponent, ComponentType};

#[component]
pub fn DropZone() -> Element {
    let drag_context = use_drag_drop_context();
    
    // Initialize default content only once
    let mut has_initialized = use_signal(|| false);
    
    use_effect({
        let drag_context = drag_context.clone();
        move || {
            if !has_initialized() {
                let layout_components: Vec<_> = drag_context
                    .get_components_in_drop_zone()
                    .into_iter()
                    .filter(|c| matches!(c.component_type, 
                        ComponentType::OneColumn | ComponentType::TwoColumns | ComponentType::ThreeColumns |
                        ComponentType::FourColumns | ComponentType::FiveColumns))
                    .collect();
                
                // If no layout exists, create a default single column layout with pre-existing content
                if layout_components.is_empty() {
                    // Add default single column layout
                    let default_layout_id = drag_context.add_component(ComponentType::OneColumn);
                    drag_context.move_to_drop_zone(&default_layout_id, (20.0, 20.0));
                    
                    // Add welcome header
                    let header_id = drag_context.add_component(ComponentType::Header);
                    drag_context.update_component_name(&header_id, "Welcome");
                    drag_context.add_to_column(&header_id, &default_layout_id, 0);
                    
                    // Add lorem ipsum text
                    let text_id = drag_context.add_component(ComponentType::Text);
                    drag_context.update_component_name(&text_id, "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.");
                    drag_context.add_to_column(&text_id, &default_layout_id, 0);
                    
                    has_initialized.set(true);
                }
            }
        }
    });
    
    let drag_context_drop = drag_context.clone();
    let drag_context_html = drag_context.clone();
    let drag_context_rsx = drag_context.clone();
    
    rsx! {
        div {
            id: "drop-zone",
            class: "flex-1 p-4 border-2 border-dashed border-gray-300 min-h-96 relative bg-gray-50 rounded-lg",
            ondragover: move |e| e.prevent_default(),
            ondrop: move |e| {
                e.prevent_default();
                if let Some(dragged_id) = drag_context_drop.take_currently_dragging() {
                    let all_components = drag_context_drop.get_components_in_palette();
                    let in_drop_zone_components = drag_context_drop
                        .get_components_in_drop_zone();
                    if let Some(dragged_component) = all_components
                        .iter()
                        .chain(in_drop_zone_components.iter())
                        .find(|c| c.id == dragged_id)
                    {
                        match dragged_component.component_type {
                            ComponentType::OneColumn
                            | ComponentType::TwoColumns
                            | ComponentType::ThreeColumns
                            | ComponentType::FourColumns
                            | ComponentType::FiveColumns => {
                                let existing_layouts = drag_context_drop
                                    .get_components_in_drop_zone()
                                    .into_iter()
                                    .filter(|c| c.parent_id.is_none())
                                    .filter(|c| {
                                        matches!(
                                            c.component_type,
                                            ComponentType::OneColumn
                                            | ComponentType::TwoColumns
                                            | ComponentType::ThreeColumns
                                            | ComponentType::FourColumns
                                            | ComponentType::FiveColumns
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                let y_position = existing_layouts.len() as f64 * 200.0 + 20.0;
                                drag_context_drop
                                    .move_to_drop_zone(&dragged_id, (20.0, y_position));
                            }
                            _ => {
                                web_sys::console::log_1(
                                    &"Content components must be dropped into column areas!"
                                        .into(),
                                );
                            }
                        }
                    }
                }
            },
            // Header
            div { class: "text-center text-gray-500 mb-4",
                h2 { class: "text-2xl font-semibold mb-2", "Website Builder Canvas" }
                p { "Drag layout components here first, then add content to the columns" }
            }
            // Export buttons
            div { class: "absolute top-4 right-4 flex gap-2",
                button {
                    class: "px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600",
                    onclick: move |_| {
                        let html = drag_context_html.export_to_html();
                        web_sys::console::log_1(&format!("HTML Export:\n{}", html).into());
                    },
                    "Export HTML"
                }
                button {
                    class: "px-3 py-1 bg-green-500 text-white rounded text-sm hover:bg-green-600",
                    onclick: move |_| {
                        let rsx = drag_context_rsx.export_to_rsx();
                        web_sys::console::log_1(&format!("RSX Export:\n{}", rsx).into());
                    },
                    "Export RSX"
                }
            }
            // Render layout components vertically stacked - GrapeJS style preview
            div { class: "space-y-4 mt-8 bg-white rounded-lg shadow-sm border min-h-96",
                {
                    drag_context
                        .get_components_in_drop_zone()
                        .into_iter()
                        .filter(|c| c.parent_id.is_none())
                        .filter(|c| {
                            matches!(
                                c.component_type,
                                ComponentType::OneColumn
                                | ComponentType::TwoColumns
                                | ComponentType::ThreeColumns
                                | ComponentType::FourColumns
                                | ComponentType::FiveColumns
                            )
                        })
                        .map(|component| {
                            let component_id = component.id.clone();
                            rsx! {
                                DroppedLayoutComponent { key: "{component_id}", component: component.clone() }
                            }
                        })
                }
            }
        }
    }
}

#[component]
pub fn DroppedLayoutComponent(component: DraggableComponent) -> Element {
    let drag_context = use_drag_drop_context();
    let component_id = component.id.clone();
    let component_id_for_delete = component.id.clone();
    let drag_context_delete = drag_context.clone();
    
    rsx! {
        div {
            class: "bg-white border border-gray-200 rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow w-full relative group",
            draggable: "true",
            ondragstart: move |_e| {
                drag_context.set_currently_dragging(Some(component_id.clone()));
            },
            // Delete button - only visible on hover
            button {
                class: "absolute -top-2 -right-2 w-6 h-6 bg-red-500 text-white rounded-full text-xs hover:bg-red-600 opacity-0 group-hover:opacity-100 transition-opacity z-10",
                onclick: move |_| {
                    drag_context_delete.remove_component(&component_id_for_delete);
                },
                "×"
            }
            // Layout header
            div { class: "flex items-center justify-between mb-3 pb-2 border-b border-gray-100",
                h3 { class: "text-sm font-medium text-gray-700", "{component.name}" }
                span { class: "text-xs text-gray-400 bg-gray-100 px-2 py-1 rounded",
                    {
                        match component.component_type {
                            ComponentType::OneColumn => "1 Column",
                            ComponentType::TwoColumns => "2 Columns",
                            ComponentType::ThreeColumns => "3 Columns",
                            ComponentType::FourColumns => "4 Columns",
                            ComponentType::FiveColumns => "5 Columns",
                            _ => "Layout",
                        }
                    }
                }
            }
            // Column layout content - more GrapeJS-like
            {render_layout_content(&component)}
        }
    }
}

fn render_layout_content(component: &DraggableComponent) -> Element {
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
                div { class: "w-full",
                    // GrapeJS-style preview with actual rendered content
                    div { class: "grid {grid_class} gap-3 min-h-32 w-full",
                        {
                            (0..column_count)
                                .map(|i| {
                                    let column_id = format!("{}-col-{}", component.id, i);
                                    rsx! {
                                        ColumnDropZone {
                                            key: "{column_id}",
                                            column_id,
                                            parent_component_id: component.id.clone(),
                                            column_index: i,
                                        }
                                    }
                                })
                        }
                    }
                }
            }
        },
        _ => rsx! {
            div { class: "p-2 text-gray-500 text-sm", "This is not a layout component" }
        }
    }
}

#[component]
pub fn ColumnDropZone(column_id: String, parent_component_id: String, column_index: usize) -> Element {
    let drag_context = use_drag_drop_context();
    let drag_context_render = drag_context.clone();
    let drag_context_check = drag_context.clone();
    
    rsx! {
        div {
            id: "{column_id}",
            class: "border border-dashed border-gray-200 min-h-32 p-3 bg-gray-50 rounded hover:border-blue-400 hover:bg-blue-50 transition-all w-full relative",
            ondragover: move |e| e.prevent_default(),
            ondrop: move |e| {
                e.prevent_default();
                if let Some(dragged_id) = drag_context.take_currently_dragging() {
                    let all_components = drag_context.get_components_in_palette();
                    let in_drop_zone_components = drag_context.get_components_in_drop_zone();
                    if let Some(dragged_component) = all_components
                        .iter()
                        .chain(in_drop_zone_components.iter())
                        .find(|c| c.id == dragged_id)
                    {
                        match dragged_component.component_type {
                            ComponentType::OneColumn
                            | ComponentType::TwoColumns
                            | ComponentType::ThreeColumns
                            | ComponentType::FourColumns
                            | ComponentType::FiveColumns => {
                                web_sys::console::log_1(
                                    &"Layout components cannot be dropped into columns!".into(),
                                );
                            }
                            _ => {
                                drag_context
                                    .add_to_column(
                                        &dragged_id,
                                        &parent_component_id,
                                        column_index,
                                    );
                            }
                        }
                    }
                }
            },
            // Show column children - GrapeJS style rendering
            {
                drag_context_render
                    .get_components_in_column(&parent_component_id)
                    .into_iter()
                    .filter(|c| {
                        let column_count = drag_context_render
                            .get_column_count(&parent_component_id);
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
            // Empty state - GrapeJS style
            {
                let components_in_column = drag_context_check
                    .get_components_in_column(&parent_component_id)
                    .into_iter()
                    .filter(|c| {
                        let column_count = drag_context_check
                            .get_column_count(&parent_component_id);
                        let column_width = 100.0 / column_count as f64;
                        let component_column = (c.position.0 / column_width) as usize;
                        component_column == column_index
                    })
                    .collect::<Vec<_>>();
                if components_in_column.is_empty() {
                    rsx! {
                        div { class: "text-xs text-gray-400 text-center py-8 flex flex-col items-center justify-center h-full min-h-24",
                            div { class: "w-8 h-8 border-2 border-dashed border-gray-300 rounded mb-2 flex items-center justify-center",
                                "+"
                            }
                            "Drop content here"
                            span { class: "text-xs text-gray-300 mt-1", "Column {column_index + 1}" }
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

#[component]
pub fn ColumnComponent(component: DraggableComponent) -> Element {
    let drag_context = use_drag_drop_context();
    let drag_context_delete = drag_context.clone();
    let component_id = component.id.clone();
    let component_id_for_delete = component.id.clone();
    
    rsx! {
        div {
            class: "relative cursor-pointer transition-all duration-200 border border-transparent rounded mb-2 hover:border-blue-300 hover:bg-blue-50",
            draggable: "true",
            ondragstart: move |_e| {
                drag_context.set_currently_dragging(Some(component_id.clone()));
            },
            // Component toolbar - appears on hover
            div { class: "absolute -top-6 left-0 bg-blue-600 text-white text-xs px-2 py-1 rounded-t items-center gap-1 z-10 opacity-0 hover:opacity-100 group-hover:opacity-100 transition-opacity",
                span { class: "font-medium", "{component.component_type:?}" }
                button {
                    class: "ml-1 hover:bg-blue-700 px-1 rounded",
                    onclick: move |e| {
                        e.stop_propagation();
                        drag_context_delete.remove_component(&component_id_for_delete);
                    },
                    "×"
                }
            }
            // Component content - render like real website content
            {render_column_component_content(&component)}
        }
    }
}

fn render_column_component_content(component: &DraggableComponent) -> Element {
    match &component.component_type {
        ComponentType::Header => rsx! {
            h1 { class: "text-2xl font-bold text-gray-800", "{component.name}" }
        },
        ComponentType::Hero => rsx! {
            div { class: "text-center p-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded",
                h2 { class: "text-xl font-bold mb-2", "{component.name}" }
                p { class: "text-sm", "This is a hero section" }
            }
        },
        ComponentType::Text => rsx! {
            p { class: "text-gray-700 leading-relaxed", "{component.name}" }
        },
        ComponentType::Button => rsx! {
            button { class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                "{component.name}"
            }
        },
        ComponentType::Card => rsx! {
            div { class: "p-4 bg-white border border-gray-200 rounded-lg shadow-sm",
                h3 { class: "text-lg font-semibold text-gray-800 mb-2", "{component.name}" }
                p { class: "text-sm text-gray-600", "Card content goes here" }
            }
        },
        ComponentType::Footer => rsx! {
            div { class: "p-3 bg-gray-800 text-white text-center rounded", "{component.name}" }
        },
        ComponentType::Image => rsx! {
            div { class: "w-full h-32 bg-gray-200 rounded flex items-center justify-center",
                svg {
                    class: "w-8 h-8 text-gray-400",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
                    }
                }
            }
        },
        ComponentType::Form => rsx! {
            div { class: "p-4 border border-gray-200 rounded-lg",
                div { class: "mb-3",
                    label { class: "block text-sm font-medium mb-1", "Input Field" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded",
                        r#type: "text",
                        placeholder: "Enter text here...",
                    }
                }
                button { class: "px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600",
                    "Submit"
                }
            }
        },
        _ => rsx! {
            div { class: "p-2 border border-gray-300 rounded text-gray-600", "{component.name}" }
        },
    }
}
