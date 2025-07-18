use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Page {
    id: usize,
    name: String,
    active: bool,
}

#[derive(Clone, PartialEq)]
struct Layer {
    id: usize,
    name: String,
    element_type: String,
    classes: String,
    children: Vec<Layer>,
}

#[component]
pub fn PagesLayersPanel() -> Element {
    let mut pages = use_signal(|| vec![
        Page { id: 1, name: "Home".into(), active: true },
        Page { id: 2, name: "About".into(), active: false },
    ]);

    let mut layers = use_signal(|| vec![
        Layer {
            id: 1,
            name: "Header".into(),
            element_type: "header".into(),
            classes: "bg-white shadow-md".into(),
            children: vec![
                Layer {
                    id: 2,
                    name: "Logo".into(),
                    element_type: "div".into(),
                    classes: "text-xl font-bold".into(),
                    children: vec![],
                },
                Layer {
                    id: 3,
                    name: "Navigation".into(),
                    element_type: "nav".into(),
                    classes: "flex space-x-4".into(),
                    children: vec![],
                },
            ],
        },
        Layer {
            id: 4,
            name: "Main Content".into(),
            element_type: "main".into(),
            classes: "container mx-auto p-4".into(),
            children: vec![
                Layer {
                    id: 5,
                    name: "Hero Section".into(),
                    element_type: "section".into(),
                    classes: "bg-gray-100 rounded-lg p-8".into(),
                    children: vec![],
                },
            ],
        },
    ]);

    let mut new_page_name = use_signal(|| String::new());
    let mut selected_layer = use_signal(|| None::<usize>);

    rsx! {
        div { class: "flex flex-col h-full",
            // Pages Section
            div { class: "border-b border-gray-200 pb-4 mb-4",
                div { class: "flex justify-between items-center mb-4",
                    h2 { class: "text-xl font-semibold", "Pages" }
                    button {
                        class: "px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm",
                        onclick: move |_| {
                            let new_id = pages.read().last().map_or(1, |p| p.id + 1);
                            pages.write().push(Page {
                                id: new_id,
                                name: format!("Page {}", new_id),
                                active: false,
                            });
                        },
                        "Add Page"
                    }
                }

                // Pages List
                div { class: "space-y-2",
                    for page in pages.read().iter() {
                        div {
                            key: "{page.id}",
                            class: "flex justify-between items-center p-2 rounded-lg hover:bg-gray-100",
                            class: if page.active { "bg-blue-50" } else { "" },
                            onclick: move |_| {
                                pages.with_mut(|pages| {
                                    for p in pages.iter_mut() {
                                        p.active = p.id == page.id;
                                    }
                                });
                            },
                            span { "{page.name}" }
                            button {
                                class: "text-red-500 hover:text-red-700 p-1",
                                onclick: move |_| {
                                    if pages.read().len() > 1 {
                                        pages.write().retain(|p| p.id != page.id);
                                    }
                                },
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16",
                                    }
                                }
                            }
                        }
                    }
                }

                // Add New Page Form
                div { class: "mt-4 flex space-x-2",
                    input {
                        class: "flex-1 p-2 border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500",
                        placeholder: "New page name",
                        value: "{new_page_name}",
                        oninput: move |e| new_page_name.set(e.value().clone()),
                    }
                    button {
                        class: "px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                        onclick: move |_| {
                            if !new_page_name().is_empty() {
                                let new_id = pages.read().last().map_or(1, |p| p.id + 1);
                                pages.write().push(Page {
                                    id: new_id,
                                    name: new_page_name().clone(),
                                    active: false,
                                });
                                new_page_name.set(String::new());
                            }
                        },
                        "Add"
                    }
                }
            }

            // Layers Section
            div { class: "flex-1 overflow-hidden flex flex-col",
                h2 { class: "text-xl font-semibold mb-4", "Layers" }
                div { class: "flex-1 overflow-auto",
                    // Layers Tree View
                    LayerTree {
                        layers: layers.clone(),
                        selected_layer: selected_layer.clone(),
                        depth: 0,
                    }
                }

                // Layer Details Panel
                if let Some(layer_id) = selected_layer() {
                    if let Some(layer) = find_layer(&layers(), layer_id) {
                        div { class: "mt-4 p-4 border-t border-gray-200",
                            h3 { class: "font-medium mb-2", "Layer Details" }
                            div { class: "space-y-3",
                                div {
                                    p { class: "text-sm text-gray-500", "Element Type" }
                                    p { class: "font-mono bg-gray-100 p-2 rounded", "{layer.element_type}" }
                                }
                                div {
                                    p { class: "text-sm text-gray-500", "Classes" }
                                    p { class: "font-mono bg-gray-100 p-2 rounded", "{layer.classes}" }
                                }
                                div {
                                    p { class: "text-sm text-gray-500", "HTML Preview" }
                                    div { class: "font-mono bg-gray-100 p-2 rounded text-xs overflow-x-auto",
                                        {
                                            let preview = format!(
                                                "<{element_type} class=\"{classes}\">\n  /* Children would go here */\n</{element_type}>",
                                                element_type = layer.element_type,
                                                classes = layer.classes
                                            );
                                            preview
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LayerTree(layers: Signal<Vec<Layer>>, selected_layer: Signal<Option<usize>>, depth: usize) -> Element {
    rsx! {
        ul { class: "space-y-1",
            for layer in layers.read().iter() {
                li {
                    key: "{layer.id}",
                    div {
                        class: "flex items-center p-2 rounded-lg hover:bg-gray-100 cursor-pointer",
                        class: if selected_layer() == Some(layer.id) { "bg-blue-50" } else { "" },
                        style: "padding-left: {}px", depth * 16 + 8,
                        onclick: move |_| selected_layer.set(Some(layer.id)),
                        svg {
                            class: "w-4 h-4 mr-2 text-gray-500 flex-shrink-0",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: if layer.children.is_empty() {
                                    "M7 20l4-16m2 16l4-16M6 9h14M4 15h14"
                                } else {
                                    "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                                },
                            }
                        }
                        span { "{layer.name}" }
                        span { class: "ml-2 text-xs text-gray-500", "<{layer.element_type}>" }
                    }
                    if !layer.children.is_empty() {
                        LayerTree {
                            layers: Signal::new(layer.children.clone()),
                            selected_layer: selected_layer.clone(),
                            depth: depth + 1,
                        }
                    }
                }
            }
        }
    }
}

fn find_layer(layers: &[Layer], id: usize) -> Option<Layer> {
    for layer in layers {
        if layer.id == id {
            return Some(layer.clone());
        }
        if let Some(found) = find_layer(&layer.children, id) {
            return Some(found);
        }
    }
    None
}