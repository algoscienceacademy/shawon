use shawon::{
    Application, Window,
    QGraphicsScene, QGraphicsView,
    QGraphicsRectItem, QGraphicsEllipseItem, QGraphicsLineItem, QGraphicsPathItem,
    PenStyle, BrushStyle
};

fn main() {
    // Initialize the application
    let app = Application::new();
    
    // Create a main window
    let window = Window::new("Graphics Items Example", 800, 600);
    
    // Create a graphics scene
    let scene = QGraphicsScene::new();
    scene.set_background_color("white");
    
    // Create a rectangle
    let rect = QGraphicsRectItem::new(0.0, 0.0, 100.0, 80.0);
    rect.set_pen("black", 2.0, PenStyle::SolidLine);
    rect.set_brush("lightblue", BrushStyle::SolidPattern);
    rect.set_pos(100.0, 50.0);
    
    // Create an ellipse
    let ellipse = QGraphicsEllipseItem::new(0.0, 0.0, 80.0, 80.0);
    ellipse.set_pen("darkblue", 2.0, PenStyle::DashLine);
    ellipse.set_brush("lightgreen", BrushStyle::SolidPattern);
    ellipse.set_pos(250.0, 50.0);
    
    // Create a line
    let line = QGraphicsLineItem::new(0.0, 0.0, 150.0, 150.0);
    line.set_pen("red", 3.0, PenStyle::SolidLine);
    line.set_pos(400.0, 50.0);
    
    // Create a custom path
    let path = QGraphicsPathItem::new();
    path.move_to(0.0, 0.0);
    path.line_to(50.0, 0.0);
    path.line_to(50.0, 50.0);
    path.curve_to(50.0, 80.0, 20.0, 100.0, 0.0, 80.0);
    path.close_path();
    path.set_pen("purple", 2.0, PenStyle::SolidLine);
    path.set_brush("yellow", BrushStyle::SolidPattern);
    path.set_pos(100.0, 200.0);
    
    // Add items to scene
    scene.add_item(&rect);
    scene.add_item(&ellipse);
    scene.add_item(&line);
    scene.add_item(&path);
    
    // Create a view for the scene
    let view = QGraphicsView::new(&scene);
    
    // Add view to window
    window.add_graphics_view(&view);
    
    // Show the window
    window.show();
    
    // Run the application
    app.exec();
}
