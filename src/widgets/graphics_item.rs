use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_void};

/// A wrapper for QAbstractGraphicsShapeItem which provides basic shape functionality in a graphics scene
pub struct QAbstractGraphicsShapeItem {
    handle: *mut c_void,
}

unsafe extern "C" {
    fn shawon_create_graphics_rect_item(x: c_double, y: c_double, width: c_double, height: c_double) -> *mut c_void;
    fn shawon_create_graphics_ellipse_item(x: c_double, y: c_double, width: c_double, height: c_double) -> *mut c_void;
    fn shawon_create_graphics_line_item(x1: c_double, y1: c_double, x2: c_double, y2: c_double) -> *mut c_void;
    fn shawon_create_graphics_path_item() -> *mut c_void;
    fn shawon_graphics_shape_item_set_pos(item: *mut c_void, x: c_double, y: c_double);
    fn shawon_graphics_shape_item_move_by(item: *mut c_void, dx: c_double, dy: c_double);
    fn shawon_graphics_shape_item_set_rotation(item: *mut c_void, angle: c_double);
    fn shawon_graphics_shape_item_set_scale(item: *mut c_void, sx: c_double, sy: c_double);
    fn shawon_graphics_shape_item_set_visible(item: *mut c_void, visible: c_int);
    fn shawon_graphics_shape_item_set_z_value(item: *mut c_void, z: c_double);
    fn shawon_graphics_shape_item_set_pen(item: *mut c_void, color: *const c_char, width: c_double, style: c_int);
    fn shawon_graphics_shape_item_set_brush(item: *mut c_void, color: *const c_char, style: c_int);
    fn shawon_graphics_path_item_move_to(item: *mut c_void, x: c_double, y: c_double);
    fn shawon_graphics_path_item_line_to(item: *mut c_void, x: c_double, y: c_double);
    fn shawon_graphics_path_item_curve_to(item: *mut c_void, cx1: c_double, cy1: c_double, cx2: c_double, cy2: c_double, x: c_double, y: c_double);
    fn shawon_graphics_path_item_close_path(item: *mut c_void);
}

impl QAbstractGraphicsShapeItem {
    /// Set the position of the shape
    pub fn set_pos(&self, x: f64, y: f64) {
        unsafe {
            shawon_graphics_shape_item_set_pos(self.handle, x, y);
        }
    }
    
    /// Move the shape by a relative offset
    pub fn move_by(&self, dx: f64, dy: f64) {
        unsafe {
            shawon_graphics_shape_item_move_by(self.handle, dx, dy);
        }
    }
    
    /// Set the rotation angle in degrees
    pub fn set_rotation(&self, angle: f64) {
        unsafe {
            shawon_graphics_shape_item_set_rotation(self.handle, angle);
        }
    }
    
    /// Set the scale of the shape
    pub fn set_scale(&self, sx: f64, sy: f64) {
        unsafe {
            shawon_graphics_shape_item_set_scale(self.handle, sx, sy);
        }
    }
    
    /// Set whether the shape is visible
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            shawon_graphics_shape_item_set_visible(self.handle, if visible { 1 } else { 0 });
        }
    }
    
    /// Set the z-value (stacking order) of the shape
    pub fn set_z_value(&self, z: f64) {
        unsafe {
            shawon_graphics_shape_item_set_z_value(self.handle, z);
        }
    }
    
    /// Set the pen used to draw the shape's outline
    pub fn set_pen(&self, color: &str, width: f64, style: PenStyle) {
        let c_color = CString::new(color).unwrap();
        unsafe {
            shawon_graphics_shape_item_set_pen(self.handle, c_color.as_ptr(), width, style as c_int);
        }
    }
    
    /// Set the brush used to fill the shape
    pub fn set_brush(&self, color: &str, style: BrushStyle) {
        let c_color = CString::new(color).unwrap();
        unsafe {
            shawon_graphics_shape_item_set_brush(self.handle, c_color.as_ptr(), style as c_int);
        }
    }
    
    /// Get the handle to the underlying C++ object
    pub fn handle(&self) -> *mut c_void {
        self.handle
    }
}

impl AsRef<QAbstractGraphicsShapeItem> for QAbstractGraphicsShapeItem {
    fn as_ref(&self) -> &QAbstractGraphicsShapeItem {
        self
    }
}

/// A rectangle graphics item
pub struct QGraphicsRectItem {
    shape: QAbstractGraphicsShapeItem,
}

impl QGraphicsRectItem {
    /// Create a new rectangle graphics item
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        let handle = unsafe { shawon_create_graphics_rect_item(x, y, width, height) };
        QGraphicsRectItem {
            shape: QAbstractGraphicsShapeItem { handle },
        }
    }
}

impl std::ops::Deref for QGraphicsRectItem {
    type Target = QAbstractGraphicsShapeItem;
    
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsRectItem {
    fn as_ref(&self) -> &QAbstractGraphicsShapeItem {
        &self.shape
    }
}

/// An ellipse graphics item
pub struct QGraphicsEllipseItem {
    shape: QAbstractGraphicsShapeItem,
}

impl QGraphicsEllipseItem {
    /// Create a new ellipse graphics item
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        let handle = unsafe { shawon_create_graphics_ellipse_item(x, y, width, height) };
        QGraphicsEllipseItem {
            shape: QAbstractGraphicsShapeItem { handle },
        }
    }
}

impl std::ops::Deref for QGraphicsEllipseItem {
    type Target = QAbstractGraphicsShapeItem;
    
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsEllipseItem {
    fn as_ref(&self) -> &QAbstractGraphicsShapeItem {
        &self.shape
    }
}

/// A line graphics item
pub struct QGraphicsLineItem {
    shape: QAbstractGraphicsShapeItem,
}

impl QGraphicsLineItem {
    /// Create a new line graphics item
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let handle = unsafe { shawon_create_graphics_line_item(x1, y1, x2, y2) };
        QGraphicsLineItem {
            shape: QAbstractGraphicsShapeItem { handle },
        }
    }
}

impl std::ops::Deref for QGraphicsLineItem {
    type Target = QAbstractGraphicsShapeItem;
    
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsLineItem {
    fn as_ref(&self) -> &QAbstractGraphicsShapeItem {
        &self.shape
    }
}

/// A path graphics item for custom shapes
pub struct QGraphicsPathItem {
    shape: QAbstractGraphicsShapeItem,
}

impl QGraphicsPathItem {
    /// Create a new path graphics item
    pub fn new() -> Self {
        let handle = unsafe { shawon_create_graphics_path_item() };
        QGraphicsPathItem {
            shape: QAbstractGraphicsShapeItem { handle },
        }
    }
    
    /// Move to a point to start a new subpath
    pub fn move_to(&self, x: f64, y: f64) {
        unsafe {
            shawon_graphics_path_item_move_to(self.handle(), x, y);
        }
    }
    
    /// Add a line to the path from the current position
    pub fn line_to(&self, x: f64, y: f64) {
        unsafe {
            shawon_graphics_path_item_line_to(self.handle(), x, y);
        }
    }
    
    /// Add a cubic Bézier curve to the path from the current position
    pub fn curve_to(&self, cx1: f64, cy1: f64, cx2: f64, cy2: f64, x: f64, y: f64) {
        unsafe {
            shawon_graphics_path_item_curve_to(self.handle(), cx1, cy1, cx2, cy2, x, y);
        }
    }
    
    /// Close the current subpath
    pub fn close_path(&self) {
        unsafe {
            shawon_graphics_path_item_close_path(self.handle());
        }
    }
}

impl std::ops::Deref for QGraphicsPathItem {
    type Target = QAbstractGraphicsShapeItem;
    
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsPathItem {
    fn as_ref(&self) -> &QAbstractGraphicsShapeItem {
        &self.shape
    }
}

/// Pen styles for drawing outlines
#[repr(i32)]
pub enum PenStyle {
    NoPen = 0,
    SolidLine = 1,
    DashLine = 2,
    DotLine = 3,
    DashDotLine = 4,
    DashDotDotLine = 5,
}

/// Brush styles for filling shapes
#[repr(i32)]
pub enum BrushStyle {
    NoBrush = 0,
    SolidPattern = 1,
    HorPattern = 2,  // Correct name that matches Qt
    VerPattern = 3,  // Correct name that matches Qt
    CrossPattern = 4,
    DiagCrossPattern = 5,
}
