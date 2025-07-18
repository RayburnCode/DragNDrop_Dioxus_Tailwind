use dioxus::prelude::*;
use crate::Route;
use crate::components::{Footer, Navbar};
use crate::components::left::LeftNav;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "flex min-h-screen bg-gray-50",
            // Left Navigation - now self-contained in LeftNav component
            LeftNav {}
            // Main Content Area
            div { class: "flex-1 flex flex-col min-h-screen transition-all duration-200",
                // Navbar at top
                Navbar {}
                // Main content area
                main { class: "flex-1 bg-CustomBackground font-display text-CustomAccent overflow-auto",
                    div { class: "mx-auto px-6 sm:px-8 py-8 w-full max-w-full", Outlet::<Route> {} }
                }
                // Footer at bottom
                Footer {}
            }
        }
    }
}