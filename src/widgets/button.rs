use std::ffi::{CStr, CString};
use libc::{c_char, c_int, c_void};

// Let's define the core types if they don't exist
pub struct Object {
    pub handle: *mut c_void,
}

pub trait ObjectImpl {
    fn handle(&self) -> *mut c_void;
}

unsafe extern "C" {
    fn shawon_create_abstract_button() -> *mut c_void;
    fn shawon_abstract_button_set_text(button: *mut c_void, text: *const c_char);
    fn shawon_abstract_button_get_text(button: *mut c_void) -> *const c_char;
    fn shawon_abstract_button_set_checkable(button: *mut c_void, checkable: c_int);
    fn shawon_abstract_button_is_checkable(button: *mut c_void) -> c_int;
    fn shawon_abstract_button_set_checked(button: *mut c_void, checked: c_int);
    fn shawon_abstract_button_is_checked(button: *mut c_void) -> c_int;
    fn shawon_abstract_button_on_clicked(button: *mut c_void, callback: unsafe extern "C" fn(*mut c_void), user_data: *mut c_void);
}

/// A wrapper for QAbstractButton which provides basic button functionality
pub struct QAbstractButton {
    obj: Object,
}

impl ObjectImpl for QAbstractButton {
    fn handle(&self) -> *mut c_void {
        self.obj.handle
    }
}

impl QAbstractButton {
    /// Creates a new abstract button
    pub fn new() -> Self {
        let handle = unsafe { shawon_create_abstract_button() };
        Self {
            obj: Object { handle },
        }
    }

    /// Sets the text on the button
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            shawon_abstract_button_set_text(self.handle(), c_text.as_ptr());
        }
    }

    /// Gets the current text of the button
    pub fn text(&self) -> String {
        unsafe {
            let c_text = shawon_abstract_button_get_text(self.handle());
            CStr::from_ptr(c_text).to_string_lossy().into_owned()
        }
    }

    /// Sets whether the button is checkable
    pub fn set_checkable(&self, checkable: bool) {
        unsafe {
            shawon_abstract_button_set_checkable(self.handle(), if checkable { 1 } else { 0 });
        }
    }

    /// Checks if the button is checkable
    pub fn is_checkable(&self) -> bool {
        unsafe {
            shawon_abstract_button_is_checkable(self.handle()) != 0
        }
    }

    /// Sets the checked state of the button
    pub fn set_checked(&self, checked: bool) {
        unsafe {
            shawon_abstract_button_set_checked(self.handle(), if checked { 1 } else { 0 });
        }
    }

    /// Gets the checked state of the button
    pub fn is_checked(&self) -> bool {
        unsafe {
            shawon_abstract_button_is_checked(self.handle()) != 0
        }
    }

    /// Connect a callback to the clicked signal
    pub fn on_clicked<F: Fn() + 'static>(&self, callback: F) {
        let user_data = Box::into_raw(Box::new(Box::new(callback) as Box<dyn Fn()>));
        
        unsafe extern "C" fn trampoline(user_data: *mut c_void) {
            unsafe {
                let callback = &*(user_data as *const Box<dyn Fn()>);
                callback();
            }
        }
        
        unsafe {
            shawon_abstract_button_on_clicked(self.handle(), trampoline, user_data as *mut c_void);
        }
    }
}

impl Drop for QAbstractButton {
    fn drop(&mut self) {
        // The object is automatically destroyed by Qt's parent-child mechanism
        // or by explicit call to destroy()
    }
}
