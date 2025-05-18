use std::os::raw::{c_int, c_void};
use crate::widgets::graphics_scene::QGraphicsScene;

pub struct QGraphicsView {
    handle: *mut c_void,
}

unsafe extern "C" {
    fn shawon_create_graphics_view(scene: *mut c_void) -> *mut c_void;
    fn shawon_graphics_view_set_scene(view: *mut c_void, scene: *mut c_void);
    fn shawon_graphics_view_resize(view: *mut c_void, width: c_int, height: c_int);
    fn shawon_graphics_view_fit_in_view(view: *mut c_void);
}

impl QGraphicsView {
    /// Create a new graphics view
    pub fn new(scene: &QGraphicsScene) -> Self {
        let handle = unsafe { shawon_create_graphics_view(scene.handle()) };
        Self { handle }
    }
    
    /// Set the scene displayed by this view
    pub fn set_scene(&self, scene: &QGraphicsScene) {
        unsafe {
            shawon_graphics_view_set_scene(self.handle, scene.handle());
        }
    }
    
    /// Resize the view
    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            shawon_graphics_view_resize(self.handle, width, height);
        }
    }
    
    /// Fit all scene contents in the view
    pub fn fit_in_view(&self) {
        unsafe {
            shawon_graphics_view_fit_in_view(self.handle);
        }
    }
    
    /// Get the handle to the underlying C++ object
    pub fn handle(&self) -> *mut c_void {
        self.handle
    }

    /// Get the handle as a widget pointer for adding to windows
    pub fn as_widget(&self) -> *mut c_void {
        self.handle
    }
}
