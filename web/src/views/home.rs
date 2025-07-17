use dioxus::prelude::*;

use crate::views::DragDropDemo;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        DragDropDemo {}
    }
}
