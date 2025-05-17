# ShawonUI

ShawonUI is a lightweight Rust wrapper for Qt, providing a simple and ergonomic way to create cross-platform GUI applications in Rust without external dependencies.

## Features

- **Pure Rust API**: Create Qt GUIs using only Rust code
- **No Qt Dependencies**: The library handles all Qt integration transparently
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Comprehensive Widget Set**: Includes buttons, labels, text fields, checkboxes, radio buttons, and more
- **Layout Management**: Arrange widgets using layouts
- **Container Widgets**: List, Tree, and Table widgets for displaying data
- **CSS Styling**: Customize the appearance of your application using Qt's CSS-like styling system
- **Dialog Support**: File dialogs, message boxes, and more
- **Event Handling**: Connect callbacks to widget events

## Requirements

- Rust 2021 edition or newer
- Qt 5.x installed on your system
- A C++ compiler compatible with your platform

## Installation

Add ShawonUI to your `Cargo.toml`:

```toml
[dependencies]
shawonui = "0.1.0"
```

## Quick Start

```rust
use shawonui::{Application, Button, Label, Window};
use std::rc::Rc;

fn main() {
    // Initialize the application
    let app = Application::new();
    
    // Create a window
    let window = Rc::new(Window::new("Hello ShawonUI", 400, 200));
    
    // Create a label
    let label = Label::new("Hello, world!");
    window.add_label(&label);
    
    // Create a button
    let mut button = Button::new("Click Me");
    
    // Clone window for the closure
    let window_clone = Rc::clone(&window);
    
    // Set button click callback
    button.set_callback(move || {
        window_clone.show_message_box(
            shawonui::MessageBoxIcon::Information,
            "Hello",
            "Hello from ShawonUI!"
        );
    });
    
    window.add_button(&button);
    
    // Show the window
    window.show();
    
    // Run the application
    app.exec();
}
```

## Widget Types

ShawonUI provides the following widget types:

- `Window` - Main application window or container
- `Button` - Standard push button
- `Label` - Display text or images
- `TextEntry` - Single-line text input
- `CheckBox` - Checkbox with text label
- `RadioButton` - Radio button with text label
- `GroupBox` - Container with border and title
- `TabWidget` - Container with tabbed interface
- `ListWidget` - List of items
- `TreeWidget` - Hierarchical tree view
- `TableWidget` - Grid of cells
- `Slider` - Sliding value selector
- `ComboBox` - Dropdown list
- `ProgressBar` - Progress indicator
- `Splitter` - Resizable split view

## Layout Management

ShawonUI supports horizontal layouts to arrange widgets:

```rust
// Create a layout
let layout = HBoxLayout::new();

// Add widgets to the layout
layout.add_label(&label);
layout.add_button(&button);

// Add the layout to a window or group box
window.add_layout(&layout);
```

## Styling

ShawonUI supports Qt's CSS-like styling system:

```rust
// Apply a stylesheet to a widget
button.set_stylesheet("
    background-color: #0078d7;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 5px 15px;
");

// Apply a stylesheet to the entire application
app.set_stylesheet("
    QWidget {
        font-family: 'Segoe UI', Arial, sans-serif;
        font-size: 10pt;
    }
    
    QPushButton {
        background-color: #0078d7;
        color: white;
    }
");
```

## Container Widgets

### ListWidget

```rust
let list_widget = ListWidget::new();
list_widget.add_item("Item 1");
list_widget.add_item("Item 2");

list_widget.set_item_clicked_callback(|row| {
    println!("Clicked on row {}", row);
});
```

### TreeWidget

```rust
let mut tree_widget = TreeWidget::new();
tree_widget.set_headers(&["Name", "Description"]);

let parent = tree_widget.add_top_item("Parent");
let child = tree_widget.add_child_item(parent, "Child").unwrap();

tree_widget.set_item_text(parent, Some(child), 1, "Description");
```

### TableWidget

```rust
let table_widget = TableWidget::new(3, 3);
table_widget.set_horizontal_headers(&["Column 1", "Column 2", "Column 3"]);

table_widget.set_cell_text(0, 0, "Cell 1,1");
table_widget.set_cell_text(0, 1, "Cell 1,2");
```

## Dialogs

```rust
// File open dialog
if let Some(path) = window.show_open_file_dialog("Open File", "", "All Files (*.*)") {
    println!("Selected file: {}", path);
}

// Message box
window.show_message_box(
    MessageBoxIcon::Information,
    "Information",
    "This is an information message."
);
```

## How It Works

ShawonUI uses a C++ layer to interface with Qt, and exposes a pure Rust API. The C++ code is compiled during the build process, and linked with your Rust application. This approach allows for a clean Rust API while leveraging the power and flexibility of Qt.

## License

MIT License

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
