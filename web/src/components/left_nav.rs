use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum NavSection {
    Blocks,
    PagesLayers,
    Styling,
    Templates,
    Assets,
    Settings,
}

#[component]
pub fn LeftNav() -> Element {
    let mut active_section = use_signal(|| NavSection::Blocks);
    let mut is_drawer_open = use_signal(|| false);
    
    rsx! {
        div { class: "flex h-screen z-50  bg-gray-50",
            // Compact Sidebar Navigation (icons only)
            div { class: "w-16 bg-white border-r border-gray-200 flex flex-col",
                // Logo/Header
                div { class: "p-4 border-b border-gray-200 flex justify-center",
                    svg {
                        class: "w-6 h-6 text-gray-800",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4",
                        }
                    }
                }
                // Navigation Sections (icons only)
                nav { class: "flex-1 overflow-y-auto py-4 space-y-2 flex flex-col items-center",
                    // Blocks Section
                    button {
                        class: {
                            let base = "p-2 rounded-lg text-gray-700 hover:bg-gray-100";
                            if active_section() == NavSection::Blocks {
                                format!("{base} bg-blue-50 text-blue-600")
                            } else {
                                base.to_string()
                            }
                        },
                        onclick: move |_| {
                            active_section.set(NavSection::Blocks);
                            is_drawer_open.set(true);
                        },
                        title: "Blocks",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z",
                            }
                        }
                    }
                    // Other navigation buttons (similarly compact)...
                    // Pages & Layers
                    button {
                        class: {
                            let base = "p-2 rounded-lg text-gray-700 hover:bg-gray-100";
                            if active_section() == NavSection::PagesLayers {
                                format!("{base} bg-blue-50 text-blue-600")
                            } else {
                                base.to_string()
                            }
                        },
                        onclick: move |_| {
                            active_section.set(NavSection::PagesLayers);
                            is_drawer_open.set(true);
                        },
                        title: "Pages & Layers",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z",
                            }
                        }
                    }
                                // Continue with other sections (Styling, Templates, Assets)...
                }
                // Settings/Save Section
                div { class: "p-4 border-t border-gray-200 flex justify-center",
                    button {
                        class: "p-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700",
                        title: "Save Project",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z",
                            }
                        }
                    }
                }
            }
            // Sliding Drawer for Panels
            div {
                class: "fixed h-full bg-white border-r border-gray-200 shadow-lg transition-all duration-300 ease-in-out z-10",
                style: {
                    let transform = if is_drawer_open() {
                        "translateX(0)"
                    } else {
                        "translateX(-100%)"
                    };
                    format!("width: 300px; transform: {}", transform)
                },
                // Drawer Header with close button
                div { class: "p-4 border-b border-gray-200 flex justify-between items-center",
                    h2 { class: "text-lg font-semibold",
                        match active_section() {
                            NavSection::Blocks => "Blocks",
                            NavSection::PagesLayers => "Pages & Layers",
                            NavSection::Styling => "Styling",
                            NavSection::Templates => "Templates",
                            NavSection::Assets => "Assets",
                            NavSection::Settings => "Settings",
                        }
                    }
                    button {
                        class: "p-1 rounded hover:bg-gray-100",
                        onclick: move |_| is_drawer_open.set(false),
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M6 18L18 6M6 6l12 12",
                            }
                        }
                    }
                }
                // Drawer Content
                div { class: "h-full overflow-y-auto p-4",
                    match active_section() {
                        NavSection::Blocks => rsx! {
                            BlocksPanel {}
                        },
                        NavSection::PagesLayers => rsx! {
                            PagesLayersPanel {}
                        },
                        NavSection::Styling => rsx! {
                            StylingPanel {}
                        },
                        NavSection::Templates => rsx! {
                            TemplatesPanel {}
                        },
                        NavSection::Assets => rsx! {
                            AssetsPanel {}
                        },
                        NavSection::Settings => rsx! {
                            SettingsPanel {}
                        },
                    }
                }
            }
                // Main Content Area (changes based on active section)
        // div { class: "flex-1 overflow-auto p-6",
        //     match active_section() {
        //         NavSection::Blocks => rsx! {
        //             BlocksPanel {}
        //         },
        //         NavSection::PagesLayers => rsx! {
        //             PagesLayersPanel {}
        //         },
        //         NavSection::Styling => rsx! {
        //             StylingPanel {}
        //         },
        //         NavSection::Templates => rsx! {
        //             TemplatesPanel {}
        //         },
        //         NavSection::Assets => rsx! {
        //             AssetsPanel {}
        //         },
        //         NavSection::Settings => rsx! {
        //             SettingsPanel {}
        //         },
        //     }
        // }
        }
    }
}

#[component]
// Placeholder components for each section
fn BlocksPanel() -> Element {
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

#[component]
fn BlockItem(name: String, icon: String) -> Element {
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

fn PagesLayersPanel() -> Element {
    rsx! {
        div {
            h2 { class: "text-xl font-semibold mb-4", "Pages & Layers" }
                // Implementation would go here
        }
    }
}
#[component]
fn StylingPanel() -> Element {
    rsx! {
        div {
            h2 { class: "text-xl font-semibold mb-4", "Styling Options" }
                // Implementation would go here
        }
    }
}

#[component]
fn TemplatesPanel() -> Element {
    rsx! {
        div {
            h2 { class: "text-xl font-semibold mb-4", "Templates" }
                // Implementation would go here
        }
    }
}
#[component]
fn AssetsPanel() -> Element {
    rsx! {
        div {
            h2 { class: "text-xl font-semibold mb-4", "Assets" }
                // Implementation would go here
        }
    }
}

#[component]
fn SettingsPanel() -> Element {
    rsx! {
        div {
            h2 { class: "text-xl font-semibold mb-4", "Settings" }
                // Implementation would go here
        }
    }
}