use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct DraggableItem {
    id: usize,
    name: String,
    position: (f64, f64),
    in_drop_zone: bool,
}

#[component]
pub fn DragDropDemo() -> Element {
    let mut items = use_signal(|| vec![
        DraggableItem { id: 1, name: "Component 1".to_string(), position: (20.0, 20.0), in_drop_zone: false },
        DraggableItem { id: 2, name: "Component 2".to_string(), position: (20.0, 60.0), in_drop_zone: false },
        DraggableItem { id: 3, name: "Component 3".to_string(), position: (20.0, 100.0), in_drop_zone: false },
    ]);

    let mut currently_dragging = use_signal(|| None::<usize>);

    rsx! {
        div { style: "display: flex; height: 100vh;",
            // Left panel - draggable components
            div { style: "width: 200px; border-right: 1px solid #ccc; padding: 10px;",
                h2 { "Components" }
                {
                    items()
                        .into_iter()
                        .filter(|item| !item.in_drop_zone)
                        .map(|item| {
                            let item_id = item.id;
                            let item_name = item.name.clone();
                            rsx! {
                                div {
                                    key: "{item_id}",
                                    style: "padding: 8px; margin: 4px; background: #eee; cursor: grab; user-select: none;",
                                    draggable: "true",
                                    ondragstart: move |_e| {
                                        currently_dragging.set(Some(item_id));
                                    },
                                    "{item_name}"
                                }
                            }
                        })
                }
            }
            // Right panel - drop zone
            div {
                id: "drop-zone",
                style: "flex: 1; padding: 10px; border: 2px dashed #aaa; min-height: 200px; position: relative;",
                ondragover: move |e| e.prevent_default(),
                ondrop: move |e| {
                    e.prevent_default();
                    if let Some(dragged_id) = currently_dragging.take() {
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Some(drop_zone_elem) = document.get_element_by_id("drop-zone")
                                {
                                    let drop_zone_rect = drop_zone_elem.get_bounding_client_rect();
                                    let point = e.client_coordinates();
                                    let x = point.x - drop_zone_rect.left() - 10.0;
                                    let y = point.y - drop_zone_rect.top() - 40.0;
                                    items
                                        .with_mut(|items| {
                                            if let Some(item) = items
                                                .iter_mut()
                                                .find(|i| i.id == dragged_id)
                                            {
                                                item.position = (x.max(0.0), y.max(0.0));
                                                item.in_drop_zone = true;
                                            }
                                        });
                                }
                            }
                        }
                    }
                },
                h2 { "Drop Zone" }
                {
                    items()
                        .into_iter()
                        .filter(|item| item.in_drop_zone)
                        .map(|item| {
                            let item_id = item.id;
                            let item_name = item.name.clone();
                            let x = item.position.0;
                            let y = item.position.1;
                            rsx! {
                                div {
                                    key: "{item_id}",
                                    style: "position: absolute; left: {x}px; top: {y}px; padding: 8px; background: #e0f7fa; border: 1px solid #4dd0e1; cursor: move;",
                                    draggable: "true",
                                    ondragstart: move |_e| {
                                        currently_dragging.set(Some(item_id));
                                    },
                                    "{item_name}"
                                }
                            }
                        })
                }
            }
        }
    }}
