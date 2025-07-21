use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DraggableComponent {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub position: (f64, f64),
    pub in_drop_zone: bool,
    pub properties: ComponentProperties,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    Header,
    Hero,
    Text,
    Image,
    Button,
    Form,
    Card,
    Footer,
    OneColumn,
    TwoColumns,
    ThreeColumns,
    FourColumns,
    FiveColumns,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentProperties {
    pub styles: Vec<String>,
    pub content: Option<String>,
    pub attributes: std::collections::HashMap<String, String>,
}

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            styles: vec![],
            content: None,
            attributes: std::collections::HashMap::new(),
        }
    }
}

impl ComponentType {
    pub fn icon(&self) -> &'static str {
        match self {
            ComponentType::Header => "M4 6h16M4 12h16M4 18h16",
            ComponentType::Hero => "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10",
            ComponentType::Text => "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
            ComponentType::Image => "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
            ComponentType::Button => "M5 12h14M12 5l7 7-7 7",
            ComponentType::Form => "M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1",
            ComponentType::Card => "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6z",
            ComponentType::Footer => "M4 6h16M4 18h16",
            ComponentType::OneColumn => "M4 6h16M4 18h16",
            ComponentType::TwoColumns => "M4 6h16M4 18h16M10 6v12",
            ComponentType::ThreeColumns => "M4 6h16M4 18h16M10 6v12M14 6v12",
            ComponentType::FourColumns => "M4 6h16M4 18h16M10 6v12M14 6v12M18 6v12",
            ComponentType::FiveColumns => "M4 6h16M4 18h16M10 6v12M14 6v12M18 6v12M22 6v12",

            ComponentType::Custom(_) => "M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4",
        }
    }

    pub fn default_content(&self) -> &str {
        match self {
            ComponentType::Header => "Header Component",
            ComponentType::Hero => "Hero Section",
            ComponentType::Text => "Text Content",
            ComponentType::Image => "Image Placeholder",
            ComponentType::Button => "Click Me",
            ComponentType::Form => "Form Component",
            ComponentType::Card => "Card Component",
            ComponentType::Footer => "Footer Component",
            ComponentType::OneColumn => "One Column Layout",
            ComponentType::TwoColumns => "Two Column Layout",
            ComponentType::ThreeColumns => "Three Column Layout",
            ComponentType::FourColumns => "Four Column Layout",
            ComponentType::FiveColumns => "Five Column Layout",
            ComponentType::Custom(name) => name,
        }
    }
}

// Global context provider
static mut COMPONENTS: GlobalSignal<Vec<DraggableComponent>> = Signal::global(Vec::new);
static mut CURRENTLY_DRAGGING: GlobalSignal<Option<String>> = Signal::global(|| None);
static mut NEXT_ID: GlobalSignal<usize> = Signal::global(|| 1);

#[derive(Clone, PartialEq)]
pub struct DragDropContext;

impl DragDropContext {
    pub fn new() -> Self {
        Self
    }

    pub fn add_component(&self, component_type: ComponentType) -> String {
        unsafe {
            let id = format!("component_{}", NEXT_ID());
            NEXT_ID.set(NEXT_ID() + 1);
            
            let component = DraggableComponent {
                id: id.clone(),
                name: component_type.default_content().to_string(),
                component_type,
                position: (20.0, 20.0),
                in_drop_zone: false,
                properties: ComponentProperties::default(),
                parent_id: None,
                children: Vec::new(),
            };

            COMPONENTS.with_mut(|components| {
                components.push(component);
            });

            id
        }
    }

    pub fn move_to_drop_zone(&self, component_id: &str, position: (f64, f64)) {
        unsafe {
            COMPONENTS.with_mut(|components| {
                if let Some(component) = components.iter_mut().find(|c| c.id == component_id) {
                    component.position = position;
                    component.in_drop_zone = true;
                }
            });
        }
    }

    pub fn update_position(&self, component_id: &str, position: (f64, f64)) {
        unsafe {
            COMPONENTS.with_mut(|components| {
                if let Some(component) = components.iter_mut().find(|c| c.id == component_id) {
                    component.position = position;
                }
            });
        }
    }

    pub fn remove_component(&self, component_id: &str) {
        unsafe {
            COMPONENTS.with_mut(|components| {
                components.retain(|c| c.id != component_id);
            });
        }
    }

    pub fn update_component_name(&self, component_id: &str, new_name: &str) {
        unsafe {
            COMPONENTS.with_mut(|components| {
                if let Some(component) = components.iter_mut().find(|c| c.id == component_id) {
                    component.name = new_name.to_string();
                }
            });
        }
    }

    pub fn get_components_in_palette(&self) -> Vec<DraggableComponent> {
        unsafe {
            COMPONENTS().into_iter().filter(|c| !c.in_drop_zone).collect()
        }
    }

    pub fn get_components_in_drop_zone(&self) -> Vec<DraggableComponent> {
        unsafe {
            COMPONENTS().into_iter().filter(|c| c.in_drop_zone).collect()
        }
    }

    pub fn set_currently_dragging(&self, component_id: Option<String>) {
        unsafe {
            CURRENTLY_DRAGGING.set(component_id);
        }
    }

    pub fn take_currently_dragging(&self) -> Option<String> {
        unsafe {
            let current = CURRENTLY_DRAGGING();
            CURRENTLY_DRAGGING.set(None);
            current
        }
    }

    pub fn add_to_column(&self, component_id: &str, column_id: &str, column_index: usize) {
        unsafe {
            COMPONENTS.with_mut(|components| {
                // Find the component and column
                let mut component_to_move = None;
                let mut column_component = None;
                
                for component in components.iter_mut() {
                    if component.id == component_id {
                        component_to_move = Some(component.clone());
                        component.parent_id = Some(column_id.to_string());
                        component.in_drop_zone = true;
                        // Calculate position within column
                        let column_width = match self.get_column_count(column_id) {
                            1 => 100.0,
                            2 => 50.0,
                            3 => 33.33,
                            4 => 25.0,
                            5 => 20.0,
                            _ => 50.0,
                        };
                        component.position = (column_index as f64 * column_width, 10.0);
                    }
                    if component.id == column_id {
                        column_component = Some(component);
                    }
                }
                
                // Add to column's children
                if let Some(column) = column_component {
                    if !column.children.contains(&component_id.to_string()) {
                        column.children.push(component_id.to_string());
                    }
                }
            });
        }
    }

    pub fn get_column_count(&self, column_id: &str) -> usize {
        unsafe {
            for component in COMPONENTS().iter() {
                if component.id == column_id {
                    return match component.component_type {
                        ComponentType::OneColumn => 1,
                        ComponentType::TwoColumns => 2,
                        ComponentType::ThreeColumns => 3,
                        ComponentType::FourColumns => 4,
                        ComponentType::FiveColumns => 5,
                        _ => 1,
                    };
                }
            }
            1
        }
    }

    pub fn get_components_in_column(&self, column_id: &str) -> Vec<DraggableComponent> {
        unsafe {
            COMPONENTS().into_iter()
                .filter(|c| c.parent_id.as_ref() == Some(&column_id.to_string()))
                .collect()
        }
    }

    pub fn export_to_html(&self) -> String {
        let components = self.get_components_in_drop_zone();
        let mut html = String::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Generated Layout</title>\n    <script src=\"https://cdn.tailwindcss.com\"></script>\n</head>\n<body>\n");
        
        for component in components.iter().filter(|c| c.parent_id.is_none()) {
            html.push_str(&self.component_to_html(component, &components));
        }
        
        html.push_str("</body>\n</html>");
        html
    }

    pub fn export_to_rsx(&self) -> String {
        let components = self.get_components_in_drop_zone();
        let mut rsx = String::from("rsx! {\n    div { class: \"min-h-screen bg-gray-50\",\n");
        
        for component in components.iter().filter(|c| c.parent_id.is_none()) {
            rsx.push_str(&self.component_to_rsx(component, &components, 2));
        }
        
        rsx.push_str("    }\n}");
        rsx
    }

    fn component_to_html(&self, component: &DraggableComponent, all_components: &[DraggableComponent]) -> String {
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
                
                let mut html = format!("<div class=\"grid {} gap-4 p-4\">\n", grid_class);
                
                let children: Vec<_> = all_components.iter()
                    .filter(|c| c.parent_id.as_ref() == Some(&component.id))
                    .collect();
                
                for i in 0..column_count {
                    html.push_str("    <div class=\"min-h-32 border-2 border-dashed border-gray-300 p-2\">\n");
                    
                    // Add children in this column
                    for child in children.iter() {
                        // Simple positioning - could be enhanced
                        if (child.position.0 / (100.0 / column_count as f64)) as usize == i {
                            html.push_str(&self.component_to_html(child, all_components));
                        }
                    }
                    
                    html.push_str("    </div>\n");
                }
                
                html.push_str("</div>\n");
                html
            },
            ComponentType::Header => format!("<h1 class=\"text-xl font-bold text-gray-800\">{}</h1>\n", component.name),
            ComponentType::Hero => format!("<div class=\"text-center p-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded\">\n    <h2 class=\"text-lg font-bold\">{}</h2>\n    <p class=\"text-sm\">This is a hero section</p>\n</div>\n", component.name),
            ComponentType::Text => format!("<p class=\"text-gray-700\">{}</p>\n", component.name),
            ComponentType::Button => format!("<button class=\"px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600\">{}</button>\n", component.name),
            ComponentType::Card => format!("<div class=\"p-3 bg-white border border-gray-200 rounded-lg shadow-sm\">\n    <h3 class=\"font-semibold text-gray-800\">{}</h3>\n    <p class=\"text-sm text-gray-600\">Card content</p>\n</div>\n", component.name),
            ComponentType::Footer => format!("<div class=\"p-2 bg-gray-800 text-white text-center rounded\">\n    <p class=\"text-sm\">{}</p>\n</div>\n", component.name),
            _ => format!("<div class=\"p-2 border border-gray-300 rounded\">{}</div>\n", component.name),
        }
    }

    fn component_to_rsx(&self, component: &DraggableComponent, all_components: &[DraggableComponent], indent: usize) -> String {
        let spaces = " ".repeat(indent);
        
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
                
                let mut rsx = format!("{}div {{ class: \"grid {} gap-4 p-4\",\n", spaces, grid_class);
                
                let children: Vec<_> = all_components.iter()
                    .filter(|c| c.parent_id.as_ref() == Some(&component.id))
                    .collect();
                
                for i in 0..column_count {
                    rsx.push_str(&format!("{}    div {{ class: \"min-h-32 border-2 border-dashed border-gray-300 p-2\",\n", spaces));
                    
                    // Add children in this column
                    for child in children.iter() {
                        if (child.position.0 / (100.0 / column_count as f64)) as usize == i {
                            rsx.push_str(&self.component_to_rsx(child, all_components, indent + 8));
                        }
                    }
                    
                    rsx.push_str(&format!("{}    }}\n", spaces));
                }
                
                rsx.push_str(&format!("{}}}\n", spaces));
                rsx
            },
            ComponentType::Header => format!("{}h1 {{ class: \"text-xl font-bold text-gray-800\", \"{}\" }}\n", spaces, component.name),
            ComponentType::Hero => format!("{}div {{ class: \"text-center p-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded\",\n{}    h2 {{ class: \"text-lg font-bold\", \"{}\" }}\n{}    p {{ class: \"text-sm\", \"This is a hero section\" }}\n{}}}\n", spaces, spaces, component.name, spaces, spaces),
            ComponentType::Text => format!("{}p {{ class: \"text-gray-700\", \"{}\" }}\n", spaces, component.name),
            ComponentType::Button => format!("{}button {{ class: \"px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600\", \"{}\" }}\n", spaces, component.name),
            ComponentType::Card => format!("{}div {{ class: \"p-3 bg-white border border-gray-200 rounded-lg shadow-sm\",\n{}    h3 {{ class: \"font-semibold text-gray-800\", \"{}\" }}\n{}    p {{ class: \"text-sm text-gray-600\", \"Card content\" }}\n{}}}\n", spaces, spaces, component.name, spaces, spaces),
            ComponentType::Footer => format!("{}div {{ class: \"p-2 bg-gray-800 text-white text-center rounded\",\n{}    p {{ class: \"text-sm\", \"{}\" }}\n{}}}\n", spaces, spaces, component.name, spaces),
            _ => format!("{}div {{ class: \"p-2 border border-gray-300 rounded\", \"{}\" }}\n", spaces, component.name),
        }
    }
}

pub fn use_drag_drop_context() -> DragDropContext {
    DragDropContext::new()
}

#[component]
pub fn DragDropProvider(children: Element) -> Element {
    // Initialize the context
    use_drag_drop_context();
    
    rsx! {
        {children}
    }
}
