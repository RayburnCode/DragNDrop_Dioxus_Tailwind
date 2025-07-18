use dioxus::prelude::*;
use log;

#[derive(Clone, PartialEq, Debug)]
struct ThemeSettings {
    primary_color: String,
    secondary_color: String,
    accent_color: String,
    button_color: String,
    link_color: String,
    text_color: String,
    background_color: String,
    custom_classes: String,
}

#[component]
pub fn StylingPanel() -> Element {
    let mut settings = use_signal(|| ThemeSettings {
        primary_color: "blue-600".into(),
        secondary_color: "gray-600".into(),
        accent_color: "indigo-600".into(),
        button_color: "blue-600".into(),
        link_color: "blue-600".into(),
        text_color: "gray-800".into(),
        background_color: "white".into(),
        custom_classes: "".into(),
    });

    let color_options = vec![
        "slate", "gray", "zinc", "neutral", "stone", 
        "red", "orange", "amber", "yellow", "lime",
        "green", "emerald", "teal", "cyan", "sky",
        "blue", "indigo", "violet", "purple", "fuchsia",
        "pink", "rose"
    ];

    let intensity_options = vec!["50", "100", "200", "300", "400", "500", "600", "700", "800", "900"];

    rsx! {
        div { class: "p-6 space-y-6",
            h2 { class: "text-2xl font-bold", "Theme Customization" }
            // Color Selection Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                // Primary Color
                ColorSelector {
                    label: "Primary Color",
                    current_value: settings().primary_color.clone(),
                    on_change: move |value| settings.write().primary_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
                // Secondary Color
                ColorSelector {
                    label: "Secondary Color",
                    current_value: settings().secondary_color.clone(),
                    on_change: move |value| settings.write().secondary_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
                // Accent Color
                ColorSelector {
                    label: "Accent Color",
                    current_value: settings().accent_color.clone(),
                    on_change: move |value| settings.write().accent_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
                // Button Color
                ColorSelector {
                    label: "Button Color",
                    current_value: settings().button_color.clone(),
                    on_change: move |value| settings.write().button_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
                // Link Color
                ColorSelector {
                    label: "Link Color",
                    current_value: settings().link_color.clone(),
                    on_change: move |value| settings.write().link_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
                // Text Color
                ColorSelector {
                    label: "Text Color",
                    current_value: settings().text_color.clone(),
                    on_change: move |value| settings.write().text_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
                // Background Color
                ColorSelector {
                    label: "Background Color",
                    current_value: settings().background_color.clone(),
                    on_change: move |value| settings.write().background_color = value,
                    color_options: color_options.clone(),
                    intensity_options: intensity_options.clone(),
                }
            }
            // Custom Tailwind Classes
            div {
                label { class: "block mb-2 text-sm font-medium text-gray-900", "Custom Tailwind Classes" }
                textarea {
                    class: "w-full p-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                    placeholder: "Enter additional Tailwind classes (e.g., rounded-lg shadow-md)",
                    rows: "3",
                    value: "{settings().custom_classes}",
                    oninput: move |e| settings.write().custom_classes = e.value().clone(),
                }
            }
            // Preview Section
            div { class: "mt-8 p-4 border rounded-lg",
                h3 { class: "text-lg font-semibold mb-4", "Theme Preview" }
                div {
                    class: "space-y-4",
                    style: "background-color: var(--background-color)",
                    // Preview Button
                    button { class: "px-4 py-2 rounded text-white bg-{settings().button_color} hover:bg-{settings().button_color}-700",
                        "Sample Button"
                    }
                    // Preview Link
                    a {
                        class: "text-{settings().link_color} hover:underline",
                        href: "#",
                        "Sample Link"
                    }
                    // Preview Text
                    p { class: "text-{settings().text_color}",
                        "Sample text showing the current text color settings"
                    }
                    // Preview Card
                    div { class: "p-4 border rounded-lg bg-{settings().background_color} border-{settings().secondary_color}",
                        h4 { class: "text-lg font-semibold text-{settings().primary_color}",
                            "Card Title"
                        }
                        p { class: "text-{settings().text_color}",
                            "This card shows how your colors work together"
                        }
                    }
                }
            }
            // Action Buttons
            div { class: "flex justify-end space-x-4 mt-6",
                button {
                    class: "px-4 py-2 bg-gray-200 text-gray-800 rounded-lg hover:bg-gray-300",
                    onclick: move |_| {
                        settings.write().primary_color = "blue-600".into();
                        settings.write().secondary_color = "gray-600".into();
                        settings.write().accent_color = "indigo-600".into();
                        settings.write().button_color = "blue-600".into();
                        settings.write().link_color = "blue-600".into();
                        settings.write().text_color = "gray-800".into();
                        settings.write().background_color = "white".into();
                        settings.write().custom_classes = "".into();
                    },
                    "Reset to Defaults"
                }
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                    onclick: move |_| {
                        log::info!("Saved theme settings: {:?}", settings.read());
                    },
                    "Save Theme"
                }
            }
        }
    }
}

#[component]
fn ColorSelector(
    label: String,
    current_value: String,
    on_change: EventHandler<String>,
    color_options: Vec<&'static str>,
    intensity_options: Vec<&'static str>,
) -> Element {
    let (current_color, current_intensity) = {
        let parts: Vec<&str> = current_value.split('-').collect();
        if parts.len() == 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ("blue".into(), "600".into())
        }
    };

    let current_color_clone = current_color.clone();
    let current_intensity_clone = current_intensity.clone();

    rsx! {
        div {
            label { class: "block mb-2 text-sm font-medium text-gray-900", "{label}" }
            div { class: "flex space-x-2",
                // Color dropdown
                select {
                    class: "flex-1 p-2 border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500",
                    value: "{current_color}",
                    onchange: move |e| {
                        let new_color = e.value().clone();
                        on_change.call(format!("{new_color}-{current_intensity_clone}"));
                    },
                    for color in color_options {
                        option {
                            value: "{color}",
                            selected: color == current_color,
                            "{color}"
                        }
                    }
                }
                // Intensity dropdown
                select {
                    class: "flex-1 p-2 border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500",
                    value: "{current_intensity}",
                    onchange: move |e| {
                        let new_intensity = e.value().clone();
                        on_change.call(format!("{current_color_clone}-{new_intensity}"));
                    },
                    for intensity in intensity_options {
                        option {
                            value: "{intensity}",
                            selected: intensity == current_intensity,
                            "{intensity}"
                        }
                    }
                }
                // Color preview
                div {
                    class: "w-10 h-10 rounded border border-gray-300",
                    style: "background-color: var(--{current_color}-{current_intensity})",
                }
            }
        }
    }
}