use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::use_route;

#[component]
pub fn Navbar(children: Element) -> Element {
    let current_route = use_route::<Route>();

    // Helper function to determine active class
    fn active_class(route: &Route, current_route: &Route, class: &str) -> String {
        if route == current_route {
            format!("{} text-CustomHover font-medium border-b-2 border-CustomHover", class)
        } else {
            class.to_string()
        }
    }

    rsx! {
        nav { class: "sticky top-0 z-40 ml-16 md:ml-20 lg:ml-24 text-CustomAccent bg-CustomNav \
                  backdrop-blur-md border-b border-gray-200 shadow-sm",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-end", // Changed to justify-end
                    // Navigation links aligned to right
                    div { class: "flex items-center space-x-6",
                        Link {
                            to: Route::Home {},
                            class: active_class(
                                &Route::Home {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Home"
                        }

                        Link {
                            to: Route::Blog { id: 1 },
                            class: active_class(
                                &Route::Blog { id: 1 },
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Blog"
                        }
                        Link {
                            to: Route::DragDropDemo {},
                            class: active_class(
                                &Route::DragDropDemo {},
                                &current_route,
                                "text-CustomAccent hover:text-CustomAccentDarker px-1 py-2 text-sm font-medium transition-colors",
                            ),
                            "Drag and Drop Demo"
                        }
                        Link {
                            to: Route::Home {},
                            class: if matches!(current_route, Route::Home {}) { "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow focus:outline-none transition-colors" } else { "ml-4 rounded-md bg-CustomHover px-4 py-2 text-sm font-medium text-CustomBackground shadow hover:bg-CustomHoverDarker focus:outline-none transition-colors" },
                            "Home Side Link"
                        }
                    }
                }
            }
        }
    }
}