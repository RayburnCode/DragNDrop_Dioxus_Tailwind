use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
struct Asset {
    id: String,
    name: String,
    file_type: String,
    preview_url: String,
    size: String,
}

#[component]
pub fn AssetPanel() -> Element {
    let mut assets = use_signal(|| Vec::<Asset>::new());
    let mut is_dragover = use_signal(|| false);
    let file_input_ref = use_node_ref();

    // Handle file uploads
    let on_file_change = move |files: Vec<FileData>| {
        for file in files {
            let file_name = file.name();
            let file_type = file_name.split('.').last().unwrap_or("file").to_string();
            let size = format!("{:.1} KB", file.size() as f64 / 1024.0);
            
            // In a real app, you would upload to a server or process the file
            let preview_url = match file_type.as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "svg" => {
                    // For images, create object URL for preview
                    file.webkit_relative_path()
                }
                _ => {
                    // For other files, use icon
                    String::new()
                }
            };

            assets.write().push(Asset {
                id: Uuid::new_v4().to_string(),
                name: file_name,
                file_type,
                preview_url,
                size,
            });
        }
    };

    // Handle drag and drop
    let ondragover = move |e: DragEvent| {
        e.prevent_default();
        is_dragover.set(true);
    };

    let ondragleave = move |_| {
        is_dragover.set(false);
    };

    let ondrop = move |e: DragEvent| {
        e.prevent_default();
        is_dragover.set(false);

        if let Some(files) = e.files() {
            on_file_change(files);
        }
    };

    // Trigger file input click
    let trigger_file_input = {
        let file_input_ref = file_input_ref.clone();
        move |_| {
            if let Some(input) = file_input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = input.click();
            }
        }
    };

    rsx! {
        div { class: "flex flex-col h-full p-4",
            h2 { class: "text-xl font-semibold mb-4", "Assets" }
            // Upload Area
            div {
                class: "border-2 border-dashed rounded-lg p-6 mb-4 text-center cursor-pointer transition-colors",
                class: if is_dragover() { "border-blue-500 bg-blue-50" } else { "border-gray-300 hover:border-blue-400" },
                ondragover,
                ondragleave,
                ondrop,
                onclick: trigger_file_input,
                svg {
                    class: "mx-auto h-12 w-12 text-gray-400",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12",
                    }
                }
                p { class: "mt-2 text-sm text-gray-600",
                    span { class: "font-medium text-blue-600 hover:text-blue-500", "Click to upload" }
                    " or drag and drop"
                }
                p { class: "text-xs text-gray-500", "PNG, JPG, GIF, SVG, MP4 up to 10MB" }
                // Hidden file input
                input {
                    r#ref: file_input_ref,
                    r#type: "file",
                    class: "hidden",
                    multiple: true,
                    accept: "image/*,video/*",
                    onchange: move |e| {
                        if let Some(files) = e.files() {
                            on_file_change(files);
                        }
                    },
                }
            }
            // Asset Grid
            div { class: "flex-1 overflow-y-auto",
                if assets.read().is_empty() {
                    div { class: "text-center py-8 text-gray-500",
                        "No assets yet. Upload some files to get started."
                    }
                } else {
                    div { class: "grid grid-cols-2 sm:grid-cols-3 gap-4",
                        for asset in assets.read().iter() {
                            AssetCard {
                                key: "{asset.id}",
                                asset: asset.clone(),
                                on_drag_start: move || {
                                    log::info!("Dragging asset: {}", asset.name);
                                },
                            }
                        }
                    }
                }
            }
            // Selected Asset Info (would show when an asset is selected)
            div { class: "border-t border-gray-200 pt-4 mt-4",
                h3 { class: "font-medium mb-2", "Asset Details" }
                p { class: "text-sm text-gray-500", "Select an asset to view details" }
            }
        }
    }
}

#[component]
fn AssetCard(asset: Asset, on_drag_start: EventHandler) -> Element {
    rsx! {
        div {
            class: "border rounded-lg overflow-hidden hover:shadow-md transition-shadow bg-white",
            draggable: "true",
            ondragstart: move |_| on_drag_start.call(()),
            // Preview
            div { class: "aspect-square bg-gray-100 flex items-center justify-center",
                if asset.file_type == "jpg" || asset.file_type == "jpeg" || asset.file_type == "png"
                    || asset.file_type == "gif"
                {
                    img {
                        class: "object-cover w-full h-full",
                        src: "{asset.preview_url}",
                        alt: "{asset.name}",
                    }
                } else if asset.file_type == "svg" {
                    div { class: "p-4",
                        svg { class: "w-full h-full", view_box: "0 0 24 24" }
                    }
                } else {
                    div { class: "text-center p-4",
                        svg {
                            class: "mx-auto h-12 w-12 text-gray-400",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z",
                            }
                        }
                        p { class: "mt-2 text-sm font-medium text-gray-900 truncate",
                            "{asset.name}"
                        }
                    }
                }
            }
            // Footer with info
            div { class: "p-2 border-t",
                div { class: "flex justify-between items-center",
                    p { class: "text-xs font-medium text-gray-900 truncate", "{asset.name}" }
                    p { class: "text-xs text-gray-500", "{asset.size}" }
                }
                p { class: "text-xs text-gray-500 uppercase mt-1", "{asset.file_type}" }
            }
        }
    }
}