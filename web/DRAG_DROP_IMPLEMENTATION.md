<!-- @format -->

# Drag & Drop Component System for Dioxus + Tailwind

## Overview

I've successfully updated the DragableItem components to work globally across the application, specifically integrating with the components in the `/src/components/left` folder structure. The left navigation now supports dragging components into a drop zone.

## Key Changes Made

### 1. Global Drag Context (`src/components/drag_context.rs`)

Created a global drag-and-drop context system that includes:

- **DraggableComponent**: A comprehensive component structure with:

  - Unique ID system
  - Component type enumeration (Header, Hero, Text, Image, Button, Form, Card, Footer, Custom)
  - Position tracking
  - Drop zone status
  - Extensible properties system

- **ComponentType**: Enum defining all available component types with:

  - Associated SVG icons for each type
  - Default content for new components
  - Type-safe component identification

- **Global State Management**: Using Dioxus GlobalSignal for:
  - Component storage
  - Currently dragging state
  - Auto-incrementing ID generation

### 2. Updated Block Components (`src/components/left/block.rs`)

- **BlockItem**: Now accepts `ComponentType` instead of string parameters
- **Drag Integration**: Automatically creates new component instances when drag starts
- **BlocksPanel**: Updated to use the new component type system

### 3. Drop Zone Component (`src/components/drop_zone.rs`)

- **DropZone**: Main canvas area for dropping components
- **DroppedComponent**: Renders individual components in the drop zone with:
  - Type-specific visual representations
  - Delete functionality
  - Re-dragging capability
  - Absolute positioning

### 4. Enhanced Layout Integration

- **Main App**: Wrapped with `DragDropProvider` for global context
- **Updated Views**: Simplified drag demo to use the new drop zone component
- **Left Navigation**: Seamlessly integrates with the new drag system

## Usage

### From Left Navigation:

1. Click on "Blocks" in the left navigation
2. Drag any component (Header, Hero, Text, Image, Button, Form) from the blocks panel
3. Drop it onto the canvas in the main area
4. Components can be re-positioned by dragging them within the drop zone
5. Delete components using the red × button

### Component Types Available:

- **Header**: Simple header component
- **Hero**: Featured section with gradient background
- **Text**: Basic text content
- **Image**: Image placeholder with icon
- **Button**: Interactive button component
- **Form**: Form with input field and submit button

## Technical Benefits

1. **Global State**: Components can be accessed from anywhere in the app
2. **Type Safety**: Strong typing prevents runtime errors
3. **Extensible**: Easy to add new component types
4. **Reusable**: Drag system works across different views
5. **Clean Architecture**: Separation of concerns between state, UI, and logic

## File Structure

```
src/
├── components/
│   ├── drag_context.rs      # Global drag state and types
│   ├── drop_zone.rs         # Canvas and component rendering
│   ├── mod.rs              # Component exports
│   └── left/
│       ├── block.rs         # Draggable block components
│       ├── left_nav.rs      # Left navigation with blocks panel
│       └── mod.rs           # Left component exports
├── views/
│   ├── drag.rs             # Simplified drag demo view
│   └── layout.rs           # App layout with drag provider
└── main.rs                 # App entry point with global context
```

## Future Enhancements

1. **Persistence**: Save/load component layouts
2. **Properties Panel**: Edit component properties
3. **Nested Components**: Support for component composition
4. **Templates**: Pre-built component templates
5. **Export**: Generate code from the visual layout
6. **Undo/Redo**: Action history management

## Usage Instructions

To use the drag-and-drop system:

1. Navigate to `/drag-drop-demo` in your application
2. Open the left navigation and click on "Blocks"
3. Drag components from the blocks panel to the canvas
4. Components will automatically snap to the drop position
5. Use the delete button (×) to remove components
6. Components can be re-dragged within the canvas

The system is now fully integrated and ready for further development and customization.
