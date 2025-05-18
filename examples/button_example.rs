use shawon::{Application, Window as QWidget, Button as QButton, Label, TextEntry};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // Initialize the application
    let app = Application::new();
    
    // Create a main window
    let window = QWidget::new("QAbstractButton Example", 300, 200);
    
    // Create buttons using Button instead of AbstractButton
    let mut button = QButton::new("Click Me");
    
    // Create a checkable button - using a regular button since we don't have checkable functionality
    let mut check_button = QButton::new("Toggle Me");
    
    // Create a counter for tracking clicks
    let counter = Rc::new(RefCell::new(0));
    
    // Connect button click handler
    let counter_clone = counter.clone();
    button.set_callback(move || {
        let mut count = counter_clone.borrow_mut();
        *count += 1;
        println!("Button clicked {} times", *count);
    });
    
    // Connect second button handler
    check_button.set_callback(move || {
        println!("Toggle button clicked");
    });
    
    // Add buttons to window
    window.add_button(&button);
    window.add_button(&check_button);
    
    // Show the window
    window.show();
    
    // Run the application
    app.exec();
}
