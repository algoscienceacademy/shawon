use std::ffi::CString;
use std::os::raw::c_void;
use crate::widgets::graphics_item::QAbstractGraphicsShapeItem;

pub struct QGraphicsScene {
    handle: *mut c_void,
}

unsafe extern "C" {
    fn shawon_create_graphics_scene() -> *mut c_void;
    fn shawon_graphics_scene_add_item(scene: *mut c_void, item: *mut c_void);
    fn shawon_graphics_scene_clear(scene: *mut c_void);
    fn shawon_graphics_scene_set_background_color(scene: *mut c_void, color: *const std::os::raw::c_char);
}

impl QGraphicsScene {
    /// Create a new graphics scene
    pub fn new() -> Self {
        let handle = unsafe { shawon_create_graphics_scene() };
        Self { handle }
    }
    
    /// Add an item to the scene
    pub fn add_item<T: AsRef<QAbstractGraphicsShapeItem>>(&self, item: &T) {
        unsafe {
            shawon_graphics_scene_add_item(self.handle, item.as_ref().handle());
        }
    }
    
    /// Clear all items from the scene
    pub fn clear(&self) {
        unsafe {
            shawon_graphics_scene_clear(self.handle);
        }
    }
    
    /// Set the background color of the scene
    pub fn set_background_color(&self, color: &str) {
        let c_color = CString::new(color).unwrap();
        unsafe {
            shawon_graphics_scene_set_background_color(self.handle, c_color.as_ptr());
        }
    }
    
    /// Get the handle to the underlying C++ object
    pub fn handle(&self) -> *mut c_void {
        self.handle
    }
}

impl AsRef<QGraphicsScene> for QGraphicsScene {
    fn as_ref(&self) -> &QGraphicsScene {
        self
    }
}
