mod styles;

use shawon::{
    Application, Button, Label, TextEntry, Window,
    CheckBox, RadioButton, GroupBox, HBoxLayout, Slider, ComboBox,
    TabWidget, ProgressBar, MessageBoxIcon, ListWidget, TreeWidget, TableWidget, Splitter,
    Styleable
};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Initialize the application
    let app = Application::new();
    
    // Apply application-wide CSS styling
    app.set_stylesheet(styles::get_app_style());
    
    // Create a main window and wrap it in Rc<RefCell> so it can be accessed from callbacks
    let window = Rc::new(Window::new("Qt Widgets Demo", 800, 600));
    
    // Create a tab widget to organize our UI
    let tab_widget = TabWidget::new();
    
    // Create the first tab for basic controls
    let basic_tab = Window::new("", 0, 0);
    
    // Add basic controls to the first tab
    let label = Label::new("Qt Wrapper Demo for Rust");
    basic_tab.add_label(&label);
    
    // Create a horizontal layout for text entry
    let text_layout = HBoxLayout::new();
    
    // Add a label and text entry to the layout
    let name_label = Label::new("Enter your name:");
    text_layout.add_label(&name_label);
    
    let text_entry = TextEntry::new();
    text_layout.add_text_entry(&text_entry);
    
    // Add the layout to the basic tab
    basic_tab.add_layout(&text_layout);
    
    // Create a greeting label
    let greeting_label = Label::new("");
    basic_tab.add_label(&greeting_label);
    
    // Create a button
    let mut button = Button::new("Say Hello");
    
    // Set up shared state that can be modified from button callback
    let text_entry_rc = Rc::new(text_entry);
    let greeting_label_rc = Rc::new(RefCell::new(greeting_label));
    
    // Clone references for the closure
    let text_entry_clone = Rc::clone(&text_entry_rc);
    let greeting_label_clone = Rc::clone(&greeting_label_rc);
    
    button.set_callback(move || {
        let name = text_entry_clone.get_text();
        if !name.is_empty() {
            greeting_label_clone.borrow().set_text(&format!("Hello, {}!", name));
        } else {
            greeting_label_clone.borrow().set_text("Please enter your name");
        }
    });
    
    basic_tab.add_button(&button);
    
    // Create the second tab for advanced controls
    let advanced_tab = Window::new("", 0, 0);
    
    // Create a group box for options
    let group_box = GroupBox::new("Options");
    
    // Add checkbox to the group box
    let mut checkbox = CheckBox::new("Enable feature");
    checkbox.as_abstract_button_mut().set_toggled_callback(|checked| {
        println!("Feature {} enabled", if checked { "is" } else { "is not" });
    });
    group_box.add_checkbox(&checkbox);
    
    // Add radio buttons to the group box
    let mut radio1 = RadioButton::new("Option 1");
    let mut radio2 = RadioButton::new("Option 2");
    
    radio1.as_abstract_button_mut().set_toggled_callback(|checked| {
        if checked {
            println!("Option 1 selected");
        }
    });
    
    radio2.as_abstract_button_mut().set_toggled_callback(|checked| {
        if checked {
            println!("Option 2 selected");
        }
    });
    
    group_box.add_radio_button(&radio1);
    group_box.add_radio_button(&radio2);
    
    // Set option 1 as default
    radio1.as_abstract_button().set_checked(true);
    
    advanced_tab.add_group_box(&group_box);
    
    // Create a slider
    let mut slider = Slider::new(true);
    slider.set_range(0, 100);
    slider.set_value(50);
    
    let slider_label = Rc::new(RefCell::new(Label::new("Value: 50")));
    advanced_tab.add_label(&slider_label.borrow());
    
    slider.set_value_changed_callback(move |value| {
        slider_label.borrow().set_text(&format!("Value: {}", value));
    });
    
    advanced_tab.add_slider(&slider);
    
    // Create a combo box
    let combo = ComboBox::new();  // Removed 'mut' as it's not needed
    combo.add_item("Item 1");
    combo.add_item("Item 2");
    combo.add_item("Item 3");
    combo.set_current_index(0);
    
    let combo_label = Rc::new(RefCell::new(Label::new("Selected: Item 1")));
    advanced_tab.add_label(&combo_label.borrow());
    
    // Create a cloneable reference to the combo box
    let combo_rc = Rc::new(RefCell::new(combo));
    let combo_clone = Rc::clone(&combo_rc);
    
    // Use the cloned reference in the callback
    combo_rc.borrow_mut().set_index_changed_callback(move |_| {
        let text = combo_clone.borrow().get_current_text();
        combo_label.borrow().set_text(&format!("Selected: {}", text));
    });
    
    advanced_tab.add_combo_box(&combo_rc.borrow());
    
    // Create the third tab for dialogs and progress
    let dialog_tab = Window::new("", 0, 0);
    
    // Add a progress bar
    let progress_bar = ProgressBar::new();
    progress_bar.set_range(0, 100);
    progress_bar.set_value(0);
    progress_bar.set_text_visible(true);
    dialog_tab.add_progress_bar(&progress_bar);
    
    // Create a horizontal layout for progress buttons
    let progress_layout = HBoxLayout::new();
    
    // Button to increment progress
    let mut increment_button = Button::new("Increment");
    let progress_bar_ref = Rc::new(RefCell::new(progress_bar));
    let progress_bar_clone = Rc::clone(&progress_bar_ref);
    
    increment_button.set_callback(move || {
        let current_value = progress_bar_clone.borrow().get_value();
        let new_value = std::cmp::min(current_value + 10, 100);
        progress_bar_clone.borrow().set_value(new_value);
    });
    
    progress_layout.add_button(&increment_button);
    
    // Button to reset progress
    let mut reset_button = Button::new("Reset");
    let progress_bar_clone2 = Rc::clone(&progress_bar_ref);
    
    reset_button.set_callback(move || {
        progress_bar_clone2.borrow().set_value(0);
    });
    
    progress_layout.add_button(&reset_button);
    dialog_tab.add_layout(&progress_layout);
    
    // Add file dialog buttons
    // No need for extra RefCell since window is already in an Rc
    
    // Open file dialog button
    let mut open_file_button = Button::new("Open File...");
    let window_clone = Rc::clone(&window);
    
    open_file_button.set_callback(move || {
        if let Some(file_path) = window_clone.show_open_file_dialog(
            "Open File",
            "",
            "All Files (*.*)"
        ) {
            window_clone.show_message_box(
                MessageBoxIcon::Information,
                "File Selected",
                &format!("You selected: {}", file_path)
            );
        }
    });
    
    dialog_tab.add_button(&open_file_button);
    
    // Save file dialog button
    let mut save_file_button = Button::new("Save File...");
    let window_clone = Rc::clone(&window);
    
    save_file_button.set_callback(move || {
        if let Some(file_path) = window_clone.show_save_file_dialog(
            "Save File",
            "",
            "All Files (*.*)"
        ) {
            window_clone.show_message_box(
                MessageBoxIcon::Information,
                "File Selected",
                &format!("You selected: {}", file_path)
            );
        }
    });
    
    dialog_tab.add_button(&save_file_button);
    
    // Message box buttons
    let message_layout = HBoxLayout::new();
    
    let mut info_button = Button::new("Information");
    let window_clone = Rc::clone(&window);
    info_button.set_callback(move || {
        window_clone.show_message_box(
            MessageBoxIcon::Information,
            "Information",
            "This is an information message."
        );
    });
    message_layout.add_button(&info_button);
    
    let mut warning_button = Button::new("Warning");
    let window_clone = Rc::clone(&window);
    warning_button.set_callback(move || {
        window_clone.show_message_box(
            MessageBoxIcon::Warning,
            "Warning",
            "This is a warning message."
        );
    });
    message_layout.add_button(&warning_button);
    
    let mut error_button = Button::new("Error");
    let window_clone = Rc::clone(&window);
    error_button.set_callback(move || {
        window_clone.show_message_box(
            MessageBoxIcon::Critical,
            "Error",
            "This is an error message."
        );
    });
    message_layout.add_button(&error_button);
    
    dialog_tab.add_layout(&message_layout);
    
    // Create a fourth tab for container widgets
    let container_tab = Window::new("", 0, 0);
    
    // Create a splitter to organize the container widgets
    let splitter = Splitter::new(true);  // Horizontal splitter
    
    // Create a list widget for the left side
    let list_widget = ListWidget::new();
    list_widget.add_item("Item 1");
    list_widget.add_item("Item 2");
    list_widget.add_item("Item 3");
    list_widget.add_item("Item 4");
    list_widget.add_item("Item 5");
    
    // Add list to splitter
    splitter.add_list_widget(&list_widget);
    
    // Create a tree widget for the middle
    let mut tree_widget = TreeWidget::new();
    tree_widget.set_headers(&["Name", "Description"]);
    
    let parent1 = tree_widget.add_top_item("Parent 1");
    tree_widget.set_item_text(parent1, None, 1, "Main category");
    
    if let Some(child1) = tree_widget.add_child_item(parent1, "Child 1") {
        tree_widget.set_item_text(parent1, Some(child1), 1, "First item");
    }
    
    if let Some(child2) = tree_widget.add_child_item(parent1, "Child 2") {
        tree_widget.set_item_text(parent1, Some(child2), 1, "Second item");
    }
    
    let parent2 = tree_widget.add_top_item("Parent 2");
    tree_widget.set_item_text(parent2, None, 1, "Second category");
    
    if let Some(child3) = tree_widget.add_child_item(parent2, "Child 3") {
        tree_widget.set_item_text(parent2, Some(child3), 1, "Third item");
    }
    
    tree_widget.expand_item(parent1);
    
    // Add tree to splitter
    splitter.add_tree_widget(&tree_widget);
    
    // Create a table widget for the right side
    let table_widget = TableWidget::new(3, 3);
    table_widget.set_horizontal_headers(&["Column 1", "Column 2", "Column 3"]);
    table_widget.set_vertical_headers(&["Row 1", "Row 2", "Row 3"]);
    
    // Fill the table with data
    for row in 0..3 {
        for col in 0..3 {
            table_widget.set_cell_text(row, col, &format!("Cell {},{}", row + 1, col + 1));
        }
    }
    
    table_widget.resize_columns_to_contents();
    
    // Add table to splitter
    splitter.add_table_widget(&table_widget);
    
    // Set the initial sizes of the splitter sections
    splitter.set_sizes(&[200, 300, 300]);
    
    // Add the splitter to the container tab
    container_tab.add_splitter(&splitter);
    
    // Set up interaction for the container widgets
    
    // List widget selection 
    let list_label = Rc::new(RefCell::new(Label::new("No item selected")));
    container_tab.add_label(&list_label.borrow());
    
    // Create an Rc wrapper around the list widget
    let list_widget_rc = Rc::new(list_widget);
    let list_widget_clone = Rc::clone(&list_widget_rc);
    
    // Use the cloned reference in the callback
    let mut list_widget_mut = ListWidget::new(); // Temporary widget to set callback
    list_widget_mut.set_item_clicked_callback(move |row| {
        if let Some(text) = list_widget_clone.get_item_text(row) {
            list_label.borrow().set_text(&format!("Selected: {}", text));
        }
    });
    
    // Tree widget selection
    let tree_label = Rc::new(RefCell::new(Label::new("No tree item selected")));
    container_tab.add_label(&tree_label.borrow());
    
    tree_widget.set_item_clicked_callback(move |parent, child| {
        if child >= 0 {
            tree_label.borrow().set_text(&format!("Selected child item at parent {}, child {}", parent, child));
        } else {
            tree_label.borrow().set_text(&format!("Selected parent item at index {}", parent));
        }
    });
    
    // Table widget selection
    let table_label = Rc::new(RefCell::new(Label::new("No cell selected")));
    container_tab.add_label(&table_label.borrow());
    
    // Create an Rc wrapper around the table widget
    let table_widget_rc = Rc::new(table_widget);
    let table_widget_clone = Rc::clone(&table_widget_rc);
    
    // Use the cloned reference in the callback
    let mut table_widget_mut = TableWidget::new(1, 1); // Temporary widget to set callback
    table_widget_mut.set_cell_clicked_callback(move |row, col| {
        if let Some(text) = table_widget_clone.get_cell_text(row, col) {
            table_label.borrow().set_text(&format!("Selected cell ({},{}): {}", row, col, text));
        }
    });
    
    // Add the container tab to the tab widget
    tab_widget.add_tab(&container_tab, "Container Widgets");
    
    // Create a fifth tab for styling options
    let style_tab = Window::new("", 0, 0);
    
    // Create a group box for style options
    let style_group = GroupBox::new("Theme Options");
    
    // Add radio buttons for theme selection
    let mut light_radio = RadioButton::new("Light Theme");
    let mut dark_radio = RadioButton::new("Dark Theme");
    
    // Set light theme as default
    light_radio.as_abstract_button().set_checked(true);
    
    // Create a reference to the app
    let app_ref = Rc::new(app);
    
    // Set up callbacks for theme switching
    let app_clone = Rc::clone(&app_ref);
    light_radio.as_abstract_button_mut().set_toggled_callback(move |checked| {
        if checked {
            app_clone.set_stylesheet(styles::get_app_style());
        }
    });
    
    let app_clone = Rc::clone(&app_ref);
    dark_radio.as_abstract_button_mut().set_toggled_callback(move |checked| {
        if checked {
            app_clone.set_stylesheet(styles::get_dark_style());
        }
    });
    
    style_group.add_radio_button(&light_radio);
    style_group.add_radio_button(&dark_radio);
    
    // Add a label explaining the styling
    let style_label = Label::new("The Qt CSS styling system allows you to customize\nthe appearance of your application using CSS-like syntax.");
    style_tab.add_label(&style_label);
    
    style_tab.add_group_box(&style_group);
    
    // Add some example widgets with custom styling
    let custom_button = Button::new("Custom Styled Button");
    custom_button.set_stylesheet("
        background-color: #9c27b0;
        color: white;
        border-radius: 8px;
        padding: 8px 16px;
        font-weight: bold;
    ");
    style_tab.add_button(&custom_button);
    
    let custom_label = Label::new("Custom Styled Label");
    custom_label.set_stylesheet("
        color: #ff5722;
        font-size: 14pt;
        font-weight: bold;
        padding: 5px;
        border: 2px solid #ff5722;
        border-radius: 5px;
    ");
    style_tab.add_label(&custom_label);
    
    // Add the styling tab to the tab widget
    tab_widget.add_tab(&style_tab, "Styling");
    
    // Add the tabs to the tab widget
    tab_widget.add_tab(&basic_tab, "Basic Controls");
    tab_widget.add_tab(&advanced_tab, "Advanced Controls");
    tab_widget.add_tab(&dialog_tab, "Dialogs & Progress");
    
    // Add the tab widget to the main window
    window.add_tab_widget(&tab_widget);
    
    // Show the main window
    window.show();
    
    // Run the application
    app_ref.exec();
}
