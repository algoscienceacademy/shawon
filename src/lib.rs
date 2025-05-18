pub mod styles;
pub mod widgets;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// FFI declarations
#[allow(non_camel_case_types)]
type RustQApplication = c_void;
#[allow(non_camel_case_types)]
type RustQWidget = c_void;
#[allow(non_camel_case_types)]
type RustQButton = c_void;
#[allow(non_camel_case_types)]
type RustQLabel = c_void;
#[allow(non_camel_case_types)]
type RustQLineEdit = c_void;
#[allow(non_camel_case_types)]
type RustQAbstractButton = c_void;
#[allow(non_camel_case_types)]
type RustQCheckBox = c_void;
#[allow(non_camel_case_types)]
type RustQRadioButton = c_void;
#[allow(non_camel_case_types)]
type RustQGroupBox = c_void;
#[allow(non_camel_case_types)]
type RustQHBoxLayout = c_void;
#[allow(non_camel_case_types)]
type RustQSlider = c_void;
#[allow(non_camel_case_types)]
type RustQComboBox = c_void;
#[allow(non_camel_case_types)]
type RustQTabWidget = c_void;
#[allow(non_camel_case_types)]
type RustQProgressBar = c_void;
#[allow(non_camel_case_types)]
type RustQListWidget = c_void;
#[allow(non_camel_case_types)]
type RustQTreeWidget = c_void;
#[allow(non_camel_case_types)]
type RustQTableWidget = c_void;
#[allow(non_camel_case_types)]
type RustQSplitter = c_void;

// Add 'unsafe' keyword here
unsafe extern "C" {
    fn create_application(argc: c_int, argv: *mut *mut c_char) -> *mut RustQApplication;
    fn exec_application(app: *mut RustQApplication) -> c_int;
    fn destroy_application(app: *mut RustQApplication);

    fn create_window(title: *const c_char, width: c_int, height: c_int) -> *mut RustQWidget;
    fn show_window(window: *mut RustQWidget);
    fn destroy_window(window: *mut RustQWidget);

    fn create_button(text: *const c_char) -> *mut RustQButton;
    fn set_button_callback(button: *mut RustQButton, callback: extern "C" fn(*mut c_void), user_data: *mut c_void);
    fn add_button_to_window(window: *mut RustQWidget, button: *mut RustQButton);
    fn destroy_button(button: *mut RustQButton);

    fn create_label(text: *const c_char) -> *mut RustQLabel;
    fn set_label_text(label: *mut RustQLabel, text: *const c_char);
    fn add_label_to_window(window: *mut RustQWidget, label: *mut RustQLabel);
    fn destroy_label(label: *mut RustQLabel);

    fn create_text_entry() -> *mut RustQLineEdit;
    fn set_text_entry_text(entry: *mut RustQLineEdit, text: *const c_char);
    fn get_text_entry_text(entry: *mut RustQLineEdit) -> *const c_char;
    fn add_text_entry_to_window(window: *mut RustQWidget, entry: *mut RustQLineEdit);
    fn destroy_text_entry(entry: *mut RustQLineEdit);
    fn free_string(str: *const c_char);

    // QAbstractButton functions
    fn set_abstract_button_text(button: *mut RustQAbstractButton, text: *const c_char);
    fn get_abstract_button_text(button: *mut RustQAbstractButton) -> *const c_char;
    fn set_abstract_button_icon(button: *mut RustQAbstractButton, path: *const c_char);
    fn set_abstract_button_checkable(button: *mut RustQAbstractButton, checkable: c_int);
    fn is_abstract_button_checked(button: *mut RustQAbstractButton) -> c_int;
    fn set_abstract_button_checked(button: *mut RustQAbstractButton, checked: c_int);
    fn set_abstract_button_clicked_callback(button: *mut RustQAbstractButton, callback: extern "C" fn(*mut c_void), user_data: *mut c_void);
    fn set_abstract_button_toggled_callback(button: *mut RustQAbstractButton, callback: extern "C" fn(*mut c_void, c_int), user_data: *mut c_void);

    // QCheckBox functions
    fn create_checkbox(text: *const c_char) -> *mut RustQCheckBox;
    fn checkbox_as_abstract_button(checkbox: *mut RustQCheckBox) -> *mut RustQAbstractButton;
    fn add_checkbox_to_window(window: *mut RustQWidget, checkbox: *mut RustQCheckBox);
    fn destroy_checkbox(checkbox: *mut RustQCheckBox);

    // QRadioButton functions
    fn create_radio_button(text: *const c_char) -> *mut RustQRadioButton;
    fn radio_button_as_abstract_button(radio: *mut RustQRadioButton) -> *mut RustQAbstractButton;
    fn add_radio_button_to_window(window: *mut RustQWidget, radio: *mut RustQRadioButton);
    fn destroy_radio_button(radio: *mut RustQRadioButton);

    // QGroupBox functions
    fn create_group_box(title: *const c_char) -> *mut RustQGroupBox;
    fn add_widget_to_group_box(group: *mut RustQGroupBox, widget: *mut c_void, widget_type: c_int);
    fn add_group_box_to_window(window: *mut RustQWidget, group: *mut RustQGroupBox);
    fn destroy_group_box(group: *mut RustQGroupBox);

    // QHBoxLayout functions
    fn create_hbox_layout() -> *mut RustQHBoxLayout;
    fn add_widget_to_hbox_layout(layout: *mut RustQHBoxLayout, widget: *mut c_void, widget_type: c_int);
    fn add_layout_to_window(window: *mut RustQWidget, layout: *mut RustQHBoxLayout);
    fn destroy_hbox_layout(layout: *mut RustQHBoxLayout);

    // QSlider functions
    fn create_slider(orientation: c_int) -> *mut RustQSlider;
    fn set_slider_range(slider: *mut RustQSlider, min: c_int, max: c_int);
    fn set_slider_value(slider: *mut RustQSlider, value: c_int);
    fn get_slider_value(slider: *mut RustQSlider) -> c_int;
    fn set_slider_value_changed_callback(slider: *mut RustQSlider, callback: extern "C" fn(*mut c_void, c_int), user_data: *mut c_void);
    fn add_slider_to_window(window: *mut RustQWidget, slider: *mut RustQSlider);
    fn destroy_slider(slider: *mut RustQSlider);

    // QComboBox functions
    fn create_combo_box() -> *mut RustQComboBox;
    fn add_combo_box_item(combo: *mut RustQComboBox, text: *const c_char);
    fn set_combo_box_current_index(combo: *mut RustQComboBox, index: c_int);
    fn get_combo_box_current_index(combo: *mut RustQComboBox) -> c_int;
    fn get_combo_box_current_text(combo: *mut RustQComboBox) -> *const c_char;
    fn set_combo_box_index_changed_callback(combo: *mut RustQComboBox, callback: extern "C" fn(*mut c_void, c_int), user_data: *mut c_void);
    fn add_combo_box_to_window(window: *mut RustQWidget, combo: *mut RustQComboBox);
    fn destroy_combo_box(combo: *mut RustQComboBox);

    // QTabWidget functions
    fn create_tab_widget() -> *mut RustQTabWidget;
    fn add_tab_to_tab_widget(tabs: *mut RustQTabWidget, widget: *mut RustQWidget, label: *const c_char);
    fn get_current_tab_index(tabs: *mut RustQTabWidget) -> c_int;
    fn set_current_tab_index(tabs: *mut RustQTabWidget, index: c_int);
    fn set_tab_changed_callback(tabs: *mut RustQTabWidget, callback: extern "C" fn(*mut c_void, c_int), user_data: *mut c_void);
    fn add_tab_widget_to_window(window: *mut RustQWidget, tabs: *mut RustQTabWidget);
    fn destroy_tab_widget(tabs: *mut RustQTabWidget);

    // QProgressBar functions
    fn create_progress_bar() -> *mut RustQProgressBar;
    fn set_progress_bar_range(bar: *mut RustQProgressBar, min: c_int, max: c_int);
    fn set_progress_bar_value(bar: *mut RustQProgressBar, value: c_int);
    fn get_progress_bar_value(bar: *mut RustQProgressBar) -> c_int;
    fn set_progress_bar_text_visible(bar: *mut RustQProgressBar, visible: c_int);
    fn add_progress_bar_to_window(window: *mut RustQWidget, bar: *mut RustQProgressBar);
    fn destroy_progress_bar(bar: *mut RustQProgressBar);

    // QListWidget functions
    fn create_list_widget() -> *mut RustQListWidget;
    fn add_list_item(list: *mut RustQListWidget, text: *const c_char);
    fn clear_list(list: *mut RustQListWidget);
    fn get_selected_list_row(list: *mut RustQListWidget) -> c_int;
    fn get_list_item_text(list: *mut RustQListWidget, row: c_int) -> *const c_char;
    fn set_list_item_clicked_callback(list: *mut RustQListWidget, callback: extern "C" fn(*mut c_void, c_int), user_data: *mut c_void);
    fn add_list_widget_to_window(window: *mut RustQWidget, list: *mut RustQListWidget);
    fn destroy_list_widget(list: *mut RustQListWidget);

    // QTreeWidget functions
    fn create_tree_widget() -> *mut RustQTreeWidget;
    fn set_tree_headers(tree: *mut RustQTreeWidget, headers: *const *const c_char, count: c_int);
    fn add_tree_top_item(tree: *mut RustQTreeWidget, text: *const c_char) -> c_int;
    fn add_tree_child_item(tree: *mut RustQTreeWidget, parent_index: c_int, text: *const c_char) -> c_int;
    fn set_tree_item_text(tree: *mut RustQTreeWidget, parent_index: c_int, child_index: c_int, column: c_int, text: *const c_char);
    fn expand_tree_item(tree: *mut RustQTreeWidget, index: c_int);
    fn set_tree_item_clicked_callback(tree: *mut RustQTreeWidget, callback: extern "C" fn(*mut c_void, c_int, c_int), user_data: *mut c_void);
    fn add_tree_widget_to_window(window: *mut RustQWidget, tree: *mut RustQTreeWidget);
    fn destroy_tree_widget(tree: *mut RustQTreeWidget);

    // QTableWidget functions
    fn create_table_widget(rows: c_int, columns: c_int) -> *mut RustQTableWidget;
    fn set_table_headers(table: *mut RustQTableWidget, headers: *const *const c_char, count: c_int, orientation: c_int);
    fn set_table_cell_text(table: *mut RustQTableWidget, row: c_int, column: c_int, text: *const c_char);
    fn get_table_cell_text(table: *mut RustQTableWidget, row: c_int, column: c_int) -> *const c_char;
    fn set_table_cell_clicked_callback(table: *mut RustQTableWidget, callback: extern "C" fn(*mut c_void, c_int, c_int), user_data: *mut c_void);
    fn resize_table_columns_to_contents(table: *mut RustQTableWidget);
    fn add_table_widget_to_window(window: *mut RustQWidget, table: *mut RustQTableWidget);
    fn destroy_table_widget(table: *mut RustQTableWidget);

    // QSplitter functions
    fn create_splitter(orientation: c_int) -> *mut RustQSplitter;
    fn add_widget_to_splitter(splitter: *mut RustQSplitter, widget: *mut c_void, widget_type: c_int);
    fn set_splitter_sizes(splitter: *mut RustQSplitter, sizes: *const c_int, count: c_int);
    fn add_splitter_to_window(window: *mut RustQWidget, splitter: *mut RustQSplitter);
    fn destroy_splitter(splitter: *mut RustQSplitter);

    // Dialog functions
    fn show_open_file_dialog(parent: *mut RustQWidget, caption: *const c_char, dir: *const c_char, filter: *const c_char) -> *const c_char;
    fn show_save_file_dialog(parent: *mut RustQWidget, caption: *const c_char, dir: *const c_char, filter: *const c_char) -> *const c_char;
    fn show_message_box(parent: *mut RustQWidget, icon: c_int, title: *const c_char, text: *const c_char);

    // CSS styling functions
    fn set_application_stylesheet(app: *mut RustQApplication, stylesheet: *const c_char);
    fn set_widget_stylesheet(widget: *mut c_void, widget_type: c_int, stylesheet: *const c_char);

    // Add a general widget function to the FFI declarations
    fn add_widget_to_window(window: *mut RustQWidget, widget: *mut c_void, widget_type: c_int);
}

// Re-export graphics items
pub use widgets::graphics_item::{
    QAbstractGraphicsShapeItem, 
    QGraphicsRectItem,
    QGraphicsEllipseItem,
    QGraphicsLineItem,
    QGraphicsPathItem,
    PenStyle,
    BrushStyle,
};

// Add new exports for graphics scene and view
pub use widgets::graphics_scene::QGraphicsScene;
pub use widgets::graphics_view::QGraphicsView;

// Wrapper structs
pub struct Application {
    ptr: *mut RustQApplication,
}

pub struct Window {
    ptr: *mut RustQWidget,
}

pub struct Button {
    ptr: *mut RustQButton,
    _callback: Option<Box<dyn FnMut()>>,
}

pub struct Label {
    ptr: *mut RustQLabel,
}

pub struct TextEntry {
    ptr: *mut RustQLineEdit,
}

pub struct AbstractButton {
    ptr: *mut RustQAbstractButton,
    _clicked_callback: Option<Box<dyn FnMut()>>,
    _toggled_callback: Option<Box<dyn FnMut(bool)>>,
}

pub struct CheckBox {
    ptr: *mut RustQCheckBox,
    abstract_button: AbstractButton,
}

pub struct RadioButton {
    ptr: *mut RustQRadioButton,
    abstract_button: AbstractButton,
}

pub struct GroupBox {
    ptr: *mut RustQGroupBox,
}

pub struct HBoxLayout {
    ptr: *mut RustQHBoxLayout,
}

pub struct Slider {
    ptr: *mut RustQSlider,
    _value_changed_callback: Option<Box<dyn FnMut(i32)>>,
}

pub struct ComboBox {
    ptr: *mut RustQComboBox,
    _index_changed_callback: Option<Box<dyn FnMut(i32)>>,
}

pub struct TabWidget {
    ptr: *mut RustQTabWidget,
    _tab_changed_callback: Option<Box<dyn FnMut(i32)>>,
}

pub struct ProgressBar {
    ptr: *mut RustQProgressBar,
}

pub struct ListWidget {
    ptr: *mut RustQListWidget,
    _item_clicked_callback: Option<Box<dyn FnMut(i32)>>,
}

pub struct TreeWidget {
    ptr: *mut RustQTreeWidget,
    _item_clicked_callback: Option<Box<dyn FnMut(i32, i32)>>,
}

pub struct TableWidget {
    ptr: *mut RustQTableWidget,
    _cell_clicked_callback: Option<Box<dyn FnMut(i32, i32)>>,
}

pub struct Splitter {
    ptr: *mut RustQSplitter,
}

// Implementation
impl Application {
    pub fn new() -> Self {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
            
        let mut c_args: Vec<*mut c_char> = args
            .iter()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect();
            
        let argc = c_args.len() as c_int;
        let argv = c_args.as_mut_ptr();
        
        let ptr = unsafe { create_application(argc, argv) };
        
        Application { ptr }
    }
    
    pub fn exec(&self) -> i32 {
        unsafe { exec_application(self.ptr) }
    }

    pub fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_application_stylesheet(self.ptr, c_stylesheet.as_ptr()) }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe { destroy_application(self.ptr) }
    }
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let c_title = CString::new(title).unwrap();
        let ptr = unsafe { create_window(c_title.as_ptr(), width, height) };
        Window { ptr }
    }
    
    pub fn show(&self) {
        unsafe { show_window(self.ptr) }
    }
    
    pub fn add_button(&self, button: &Button) {
        unsafe { add_button_to_window(self.ptr, button.ptr) }
    }
    
    pub fn add_label(&self, label: &Label) {
        unsafe { add_label_to_window(self.ptr, label.ptr) }
    }
    
    pub fn add_text_entry(&self, entry: &TextEntry) {
        unsafe { add_text_entry_to_window(self.ptr, entry.ptr) }
    }

    pub fn add_checkbox(&self, checkbox: &CheckBox) {
        unsafe { add_checkbox_to_window(self.ptr, checkbox.ptr) }
    }
    
    pub fn add_radio_button(&self, radio: &RadioButton) {
        unsafe { add_radio_button_to_window(self.ptr, radio.ptr) }
    }
    
    pub fn add_group_box(&self, group: &GroupBox) {
        unsafe { add_group_box_to_window(self.ptr, group.ptr) }
    }
    
    pub fn add_layout(&self, layout: &HBoxLayout) {
        unsafe { add_layout_to_window(self.ptr, layout.ptr) }
    }
    
    pub fn add_slider(&self, slider: &Slider) {
        unsafe { add_slider_to_window(self.ptr, slider.ptr) }
    }
    
    pub fn add_combo_box(&self, combo: &ComboBox) {
        unsafe { add_combo_box_to_window(self.ptr, combo.ptr) }
    }

    pub fn add_tab_widget(&self, tabs: &TabWidget) {
        unsafe { add_tab_widget_to_window(self.ptr, tabs.ptr) }
    }
    
    pub fn add_progress_bar(&self, bar: &ProgressBar) {
        unsafe { add_progress_bar_to_window(self.ptr, bar.ptr) }
    }
    
    pub fn add_list_widget(&self, list: &ListWidget) {
        unsafe { add_list_widget_to_window(self.ptr, list.ptr) }
    }
    
    pub fn add_tree_widget(&self, tree: &TreeWidget) {
        unsafe { add_tree_widget_to_window(self.ptr, tree.ptr) }
    }
    
    pub fn add_table_widget(&self, table: &TableWidget) {
        unsafe { add_table_widget_to_window(self.ptr, table.ptr) }
    }
    
    pub fn add_splitter(&self, splitter: &Splitter) {
        unsafe { add_splitter_to_window(self.ptr, splitter.ptr) }
    }
    
    pub fn show_open_file_dialog(&self, caption: &str, dir: &str, filter: &str) -> Option<String> {
        let c_caption = CString::new(caption).unwrap();
        let c_dir = CString::new(dir).unwrap();
        let c_filter = CString::new(filter).unwrap();
        
        unsafe {
            let c_str = show_open_file_dialog(self.ptr, c_caption.as_ptr(), c_dir.as_ptr(), c_filter.as_ptr());
            if c_str.is_null() {
                None
            } else {
                let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                free_string(c_str);
                Some(result)
            }
        }
    }
    
    pub fn show_save_file_dialog(&self, caption: &str, dir: &str, filter: &str) -> Option<String> {
        let c_caption = CString::new(caption).unwrap();
        let c_dir = CString::new(dir).unwrap();
        let c_filter = CString::new(filter).unwrap();
        
        unsafe {
            let c_str = show_save_file_dialog(self.ptr, c_caption.as_ptr(), c_dir.as_ptr(), c_filter.as_ptr());
            if c_str.is_null() {
                None
            } else {
                let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                free_string(c_str);
                Some(result)
            }
        }
    }
    
    pub fn show_message_box(&self, icon: MessageBoxIcon, title: &str, text: &str) {
        let c_title = CString::new(title).unwrap();
        let c_text = CString::new(text).unwrap();
        
        unsafe {
            show_message_box(self.ptr, icon as i32, c_title.as_ptr(), c_text.as_ptr());
        }
    }

    pub fn add_graphics_view(&self, view: &QGraphicsView) {
        unsafe {
            // We'll use a generic function to add the view as a widget
            add_widget_to_window(self.ptr, view.as_widget(), 20); // Using widget type 20 for QGraphicsView
        }
    }
}

extern "C" fn button_callback_trampoline(user_data: *mut c_void) {
    let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut()>) };
    callback();
}

impl Button {
    pub fn new(text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        let ptr = unsafe { create_button(c_text.as_ptr()) };
        Button { 
            ptr,
            _callback: None,
        }
    }
    
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        let callback_box: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        unsafe {
            set_button_callback(self.ptr, button_callback_trampoline, callback_ptr);
        }
        
        self._callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut()>) });
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        unsafe { destroy_button(self.ptr) }
    }
}

impl Label {
    pub fn new(text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        let ptr = unsafe { create_label(c_text.as_ptr()) };
        Label { ptr }
    }
    
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { set_label_text(self.ptr, c_text.as_ptr()) }
    }
}

impl Drop for Label {
    fn drop(&mut self) {
        unsafe { destroy_label(self.ptr) }
    }
}

impl TextEntry {
    pub fn new() -> Self {
        let ptr = unsafe { create_text_entry() };
        TextEntry { ptr }
    }
    
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { set_text_entry_text(self.ptr, c_text.as_ptr()) }
    }
    
    pub fn get_text(&self) -> String {
        unsafe {
            let c_str = get_text_entry_text(self.ptr);
            let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            free_string(c_str);
            result
        }
    }
}

impl Drop for TextEntry {
    fn drop(&mut self) {
        unsafe { destroy_text_entry(self.ptr) }
    }
}

// AbstractButton implementation
impl AbstractButton {
    pub fn set_text(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { set_abstract_button_text(self.ptr, c_text.as_ptr()) }
    }
    
    pub fn get_text(&self) -> String {
        unsafe {
            let c_str = get_abstract_button_text(self.ptr);
            let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            free_string(c_str);
            result
        }
    }
    
    pub fn set_icon(&self, path: &str) {
        let c_path = CString::new(path).unwrap();
        unsafe { set_abstract_button_icon(self.ptr, c_path.as_ptr()) }
    }
    
    pub fn set_checkable(&self, checkable: bool) {
        unsafe { set_abstract_button_checkable(self.ptr, if checkable { 1 } else { 0 }) }
    }
    
    pub fn is_checked(&self) -> bool {
        unsafe { is_abstract_button_checked(self.ptr) != 0 }
    }
    
    pub fn set_checked(&self, checked: bool) {
        unsafe { set_abstract_button_checked(self.ptr, if checked { 1 } else { 0 }) }
    }
    
    pub fn set_clicked_callback<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        let callback_box: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn clicked_callback_trampoline(user_data: *mut c_void) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut()>) };
            callback();
        }
        
        unsafe {
            set_abstract_button_clicked_callback(self.ptr, clicked_callback_trampoline, callback_ptr);
        }
        
        self._clicked_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut()>) });
    }
    
    pub fn set_toggled_callback<F>(&mut self, callback: F)
    where
        F: FnMut(bool) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(bool)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn toggled_callback_trampoline(user_data: *mut c_void, checked: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(bool)>) };
            callback(checked != 0);
        }
        
        unsafe {
            set_abstract_button_toggled_callback(self.ptr, toggled_callback_trampoline, callback_ptr);
        }
        
        self._toggled_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(bool)>) });
    }
}

// CheckBox implementation
impl CheckBox {
    pub fn new(text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        let ptr = unsafe { create_checkbox(c_text.as_ptr()) };
        let abstract_button_ptr = unsafe { checkbox_as_abstract_button(ptr) };
        
        CheckBox {
            ptr,
            abstract_button: AbstractButton {
                ptr: abstract_button_ptr,
                _clicked_callback: None,
                _toggled_callback: None,
            },
        }
    }
    
    pub fn as_abstract_button(&self) -> &AbstractButton {
        &self.abstract_button
    }
    
    pub fn as_abstract_button_mut(&mut self) -> &mut AbstractButton {
        &mut self.abstract_button
    }
}

impl Drop for CheckBox {
    fn drop(&mut self) {
        unsafe { destroy_checkbox(self.ptr) }
    }
}

// RadioButton implementation
impl RadioButton {
    pub fn new(text: &str) -> Self {
        let c_text = CString::new(text).unwrap();
        let ptr = unsafe { create_radio_button(c_text.as_ptr()) };
        let abstract_button_ptr = unsafe { radio_button_as_abstract_button(ptr) };
        
        RadioButton {
            ptr,
            abstract_button: AbstractButton {
                ptr: abstract_button_ptr,
                _clicked_callback: None,
                _toggled_callback: None,
            },
        }
    }
    
    pub fn as_abstract_button(&self) -> &AbstractButton {
        &self.abstract_button
    }
    
    pub fn as_abstract_button_mut(&mut self) -> &mut AbstractButton {
        &mut self.abstract_button
    }
}

impl Drop for RadioButton {
    fn drop(&mut self) {
        unsafe { destroy_radio_button(self.ptr) }
    }
}

// GroupBox implementation
impl GroupBox {
    pub fn new(title: &str) -> Self {
        let c_title = CString::new(title).unwrap();
        let ptr = unsafe { create_group_box(c_title.as_ptr()) };
        GroupBox { ptr }
    }
    
    pub fn add_button(&self, button: &Button) {
        unsafe { add_widget_to_group_box(self.ptr, button.ptr as *mut c_void, 1) }
    }
    
    pub fn add_label(&self, label: &Label) {
        unsafe { add_widget_to_group_box(self.ptr, label.ptr as *mut c_void, 2) }
    }
    
    pub fn add_text_entry(&self, entry: &TextEntry) {
        unsafe { add_widget_to_group_box(self.ptr, entry.ptr as *mut c_void, 3) }
    }
    
    pub fn add_checkbox(&self, checkbox: &CheckBox) {
        unsafe { add_widget_to_group_box(self.ptr, checkbox.ptr as *mut c_void, 4) }
    }
    
    pub fn add_radio_button(&self, radio: &RadioButton) {
        unsafe { add_widget_to_group_box(self.ptr, radio.ptr as *mut c_void, 5) }
    }
}

impl Drop for GroupBox {
    fn drop(&mut self) {
        unsafe { destroy_group_box(self.ptr) }
    }
}

// HBoxLayout implementation
impl HBoxLayout {
    pub fn new() -> Self {
        let ptr = unsafe { create_hbox_layout() };
        HBoxLayout { ptr }
    }
    
    pub fn add_button(&self, button: &Button) {
        unsafe { add_widget_to_hbox_layout(self.ptr, button.ptr as *mut c_void, 1) }
    }
    
    pub fn add_label(&self, label: &Label) {
        unsafe { add_widget_to_hbox_layout(self.ptr, label.ptr as *mut c_void, 2) }
    }
    
    pub fn add_text_entry(&self, entry: &TextEntry) {
        unsafe { add_widget_to_hbox_layout(self.ptr, entry.ptr as *mut c_void, 3) }
    }
    
    pub fn add_checkbox(&self, checkbox: &CheckBox) {
        unsafe { add_widget_to_hbox_layout(self.ptr, checkbox.ptr as *mut c_void, 4) }
    }
    
    pub fn add_radio_button(&self, radio: &RadioButton) {
        unsafe { add_widget_to_hbox_layout(self.ptr, radio.ptr as *mut c_void, 5) }
    }
    
    pub fn add_group_box(&self, group: &GroupBox) {
        unsafe { add_widget_to_hbox_layout(self.ptr, group.ptr as *mut c_void, 6) }
    }
}

impl Drop for HBoxLayout {
    fn drop(&mut self) {
        unsafe { destroy_hbox_layout(self.ptr) }
    }
}

// Slider implementation
impl Slider {
    pub fn new(horizontal: bool) -> Self {
        let ptr = unsafe { create_slider(if horizontal { 0 } else { 1 }) };
        Slider { 
            ptr,
            _value_changed_callback: None,
        }
    }
    
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { set_slider_range(self.ptr, min, max) }
    }
    
    pub fn set_value(&self, value: i32) {
        unsafe { set_slider_value(self.ptr, value) }
    }
    
    pub fn get_value(&self) -> i32 {
        unsafe { get_slider_value(self.ptr) }
    }
    
    pub fn set_value_changed_callback<F>(&mut self, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn value_changed_callback_trampoline(user_data: *mut c_void, value: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(i32)>) };
            callback(value);
        }
        
        unsafe {
            set_slider_value_changed_callback(self.ptr, value_changed_callback_trampoline, callback_ptr);
        }
        
        self._value_changed_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(i32)>) });
    }
}

impl Drop for Slider {
    fn drop(&mut self) {
        unsafe { destroy_slider(self.ptr) }
    }
}

// ComboBox implementation
impl ComboBox {
    pub fn new() -> Self {
        let ptr = unsafe { create_combo_box() };
        ComboBox { 
            ptr,
            _index_changed_callback: None,
        }
    }
    
    pub fn add_item(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { add_combo_box_item(self.ptr, c_text.as_ptr()) }
    }
    
    pub fn set_current_index(&self, index: i32) {
        unsafe { set_combo_box_current_index(self.ptr, index) }
    }
    
    pub fn get_current_index(&self) -> i32 {
        unsafe { get_combo_box_current_index(self.ptr) }
    }
    
    pub fn get_current_text(&self) -> String {
        unsafe {
            let c_str = get_combo_box_current_text(self.ptr);
            let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            free_string(c_str);
            result
        }
    }
    
    pub fn set_index_changed_callback<F>(&mut self, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn index_changed_callback_trampoline(user_data: *mut c_void, index: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(i32)>) };
            callback(index);
        }
        
        unsafe {
            set_combo_box_index_changed_callback(self.ptr, index_changed_callback_trampoline, callback_ptr);
        }
        
        self._index_changed_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(i32)>) });
    }
}

impl Drop for ComboBox {
    fn drop(&mut self) {
        unsafe { destroy_combo_box(self.ptr) }
    }
}

// TabWidget implementation
impl TabWidget {
    pub fn new() -> Self {
        let ptr = unsafe { create_tab_widget() };
        TabWidget { 
            ptr,
            _tab_changed_callback: None,
        }
    }
    
    pub fn add_tab(&self, widget: &Window, label: &str) {
        let c_label = CString::new(label).unwrap();
        unsafe { add_tab_to_tab_widget(self.ptr, widget.ptr, c_label.as_ptr()) }
    }
    
    pub fn get_current_index(&self) -> i32 {
        unsafe { get_current_tab_index(self.ptr) }
    }
    
    pub fn set_current_index(&self, index: i32) {
        unsafe { set_current_tab_index(self.ptr, index) }
    }
    
    pub fn set_tab_changed_callback<F>(&mut self, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn tab_changed_callback_trampoline(user_data: *mut c_void, index: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(i32)>) };
            callback(index);
        }
        
        unsafe {
            set_tab_changed_callback(self.ptr, tab_changed_callback_trampoline, callback_ptr);
        }
        
        self._tab_changed_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(i32)>) });
    }
}

impl Drop for TabWidget {
    fn drop(&mut self) {
        unsafe { destroy_tab_widget(self.ptr) }
    }
}

// ProgressBar implementation
impl ProgressBar {
    pub fn new() -> Self {
        let ptr = unsafe { create_progress_bar() };
        ProgressBar { ptr }
    }
    
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { set_progress_bar_range(self.ptr, min, max) }
    }
    
    pub fn set_value(&self, value: i32) {
        unsafe { set_progress_bar_value(self.ptr, value) }
    }
    
    pub fn get_value(&self) -> i32 {
        unsafe { get_progress_bar_value(self.ptr) }
    }
    
    pub fn set_text_visible(&self, visible: bool) {
        unsafe { set_progress_bar_text_visible(self.ptr, if visible { 1 } else { 0 }) }
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        unsafe { destroy_progress_bar(self.ptr) }
    }
}

// ListWidget implementation
impl ListWidget {
    pub fn new() -> Self {
        let ptr = unsafe { create_list_widget() };
        ListWidget { 
            ptr,
            _item_clicked_callback: None,
        }
    }
    
    pub fn add_item(&self, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { add_list_item(self.ptr, c_text.as_ptr()) }
    }
    
    pub fn clear(&self) {
        unsafe { clear_list(self.ptr) }
    }
    
    pub fn get_selected_row(&self) -> Option<i32> {
        let row = unsafe { get_selected_list_row(self.ptr) };
        if row >= 0 {
            Some(row)
        } else {
            None
        }
    }
    
    pub fn get_item_text(&self, row: i32) -> Option<String> {
        unsafe {
            let c_str = get_list_item_text(self.ptr, row);
            if c_str.is_null() {
                None
            } else {
                let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                free_string(c_str);
                Some(result)
            }
        }
    }
    
    pub fn set_item_clicked_callback<F>(&mut self, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(i32)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn item_clicked_callback_trampoline(user_data: *mut c_void, row: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(i32)>) };
            callback(row);
        }
        
        unsafe {
            set_list_item_clicked_callback(self.ptr, item_clicked_callback_trampoline, callback_ptr);
        }
        
        self._item_clicked_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(i32)>) });
    }
}

impl Drop for ListWidget {
    fn drop(&mut self) {
        unsafe { destroy_list_widget(self.ptr) }
    }
}

// TreeWidget implementation
impl TreeWidget {
    pub fn new() -> Self {
        let ptr = unsafe { create_tree_widget() };
        TreeWidget { 
            ptr,
            _item_clicked_callback: None,
        }
    }
    
    pub fn set_headers(&self, headers: &[&str]) {
        let c_headers: Vec<CString> = headers.iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let c_header_ptrs: Vec<*const c_char> = c_headers.iter()
            .map(|cs| cs.as_ptr())
            .collect();
        
        unsafe {
            set_tree_headers(self.ptr, c_header_ptrs.as_ptr(), c_headers.len() as c_int);
        }
    }
    
    pub fn add_top_item(&self, text: &str) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { add_tree_top_item(self.ptr, c_text.as_ptr()) }
    }
    
    pub fn add_child_item(&self, parent_index: i32, text: &str) -> Option<i32> {
        let c_text = CString::new(text).unwrap();
        let index = unsafe { add_tree_child_item(self.ptr, parent_index, c_text.as_ptr()) };
        if index >= 0 {
            Some(index)
        } else {
            None
        }
    }
    
    pub fn set_item_text(&self, parent_index: i32, child_index: Option<i32>, column: i32, text: &str) {
        let c_text = CString::new(text).unwrap();
        let child_idx = child_index.unwrap_or(-1);
        unsafe {
            set_tree_item_text(self.ptr, parent_index, child_idx, column, c_text.as_ptr());
        }
    }
    
    pub fn expand_item(&self, index: i32) {
        unsafe { expand_tree_item(self.ptr, index) }
    }
    
    pub fn set_item_clicked_callback<F>(&mut self, callback: F)
    where
        F: FnMut(i32, i32) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(i32, i32)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn item_clicked_callback_trampoline(user_data: *mut c_void, parent_index: c_int, child_index: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(i32, i32)>) };
            callback(parent_index, child_index);
        }
        
        unsafe {
            set_tree_item_clicked_callback(self.ptr, item_clicked_callback_trampoline, callback_ptr);
        }
        
        self._item_clicked_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(i32, i32)>) });
    }
}

impl Drop for TreeWidget {
    fn drop(&mut self) {
        unsafe { destroy_tree_widget(self.ptr) }
    }
}

// TableWidget implementation
impl TableWidget {
    pub fn new(rows: i32, columns: i32) -> Self {
        let ptr = unsafe { create_table_widget(rows, columns) };
        TableWidget { 
            ptr,
            _cell_clicked_callback: None,
        }
    }
    
    pub fn set_horizontal_headers(&self, headers: &[&str]) {
        let c_headers: Vec<CString> = headers.iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let c_header_ptrs: Vec<*const c_char> = c_headers.iter()
            .map(|cs| cs.as_ptr())
            .collect();
        
        unsafe {
            set_table_headers(self.ptr, c_header_ptrs.as_ptr(), c_headers.len() as c_int, 0);
        }
    }
    
    pub fn set_vertical_headers(&self, headers: &[&str]) {
        let c_headers: Vec<CString> = headers.iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let c_header_ptrs: Vec<*const c_char> = c_headers.iter()
            .map(|cs| cs.as_ptr())
            .collect();
        
        unsafe {
            set_table_headers(self.ptr, c_header_ptrs.as_ptr(), c_headers.len() as c_int, 1);
        }
    }
    
    pub fn set_cell_text(&self, row: i32, column: i32, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            set_table_cell_text(self.ptr, row, column, c_text.as_ptr());
        }
    }
    
    pub fn get_cell_text(&self, row: i32, column: i32) -> Option<String> {
        unsafe {
            let c_str = get_table_cell_text(self.ptr, row, column);
            if c_str.is_null() {
                None
            } else {
                let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                free_string(c_str);
                Some(result)
            }
        }
    }
    
    pub fn resize_columns_to_contents(&self) {
        unsafe { resize_table_columns_to_contents(self.ptr) }
    }
    
    pub fn set_cell_clicked_callback<F>(&mut self, callback: F)
    where
        F: FnMut(i32, i32) + 'static,
    {
        let callback_box: Box<Box<dyn FnMut(i32, i32)>> = Box::new(Box::new(callback));
        let callback_ptr = Box::into_raw(callback_box) as *mut c_void;
        
        extern "C" fn cell_clicked_callback_trampoline(user_data: *mut c_void, row: c_int, column: c_int) {
            let callback = unsafe { &mut *(user_data as *mut Box<dyn FnMut(i32, i32)>) };
            callback(row, column);
        }
        
        unsafe {
            set_table_cell_clicked_callback(self.ptr, cell_clicked_callback_trampoline, callback_ptr);
        }
        
        self._cell_clicked_callback = Some(unsafe { Box::from_raw(callback_ptr as *mut Box<dyn FnMut(i32, i32)>) });
    }
}

impl Drop for TableWidget {
    fn drop(&mut self) {
        unsafe { destroy_table_widget(self.ptr) }
    }
}

// Splitter implementation
impl Splitter {
    pub fn new(horizontal: bool) -> Self {
        let ptr = unsafe { create_splitter(if horizontal { 0 } else { 1 }) };
        Splitter { ptr }
    }
    
    pub fn add_widget<T>(&self, widget: &T, widget_type: i32) {
        // Use as_ptr() to get the correct pointer
        let widget_ptr = widget as *const T as *mut c_void;
        unsafe {
            add_widget_to_splitter(self.ptr, widget_ptr, widget_type);
        }
    }
    
    pub fn set_sizes(&self, sizes: &[i32]) {
        unsafe {
            set_splitter_sizes(self.ptr, sizes.as_ptr(), sizes.len() as c_int);
        }
    }
    
    pub fn add_window(&self, window: &Window) {
        self.add_widget(window, 10);
    }
    
    pub fn add_list_widget(&self, list: &ListWidget) {
        self.add_widget(list, 7);
    }
    
    pub fn add_tree_widget(&self, tree: &TreeWidget) {
        self.add_widget(tree, 8);
    }
    
    pub fn add_table_widget(&self, table: &TableWidget) {
        self.add_widget(table, 9);
    }
}

impl Drop for Splitter {
    fn drop(&mut self) {
        unsafe { destroy_splitter(self.ptr) }
    }
}

// Add an enum for message box icon types
pub enum MessageBoxIcon {
    Information = 0,
    Warning = 1,
    Critical = 2,
    Question = 3,
}

// Add a trait for styleable widgets
pub trait Styleable {
    fn set_stylesheet(&self, stylesheet: &str);
}

impl Styleable for Window {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 10, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for Button {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 1, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for Label {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 2, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for TextEntry {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 3, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for CheckBox {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 4, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for RadioButton {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 5, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for GroupBox {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 6, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for ListWidget {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 7, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for TreeWidget {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 8, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for TableWidget {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 9, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for TabWidget {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 11, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for ProgressBar {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 12, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for Splitter {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 13, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for ComboBox {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 14, c_stylesheet.as_ptr()) }
    }
}

impl Styleable for Slider {
    fn set_stylesheet(&self, stylesheet: &str) {
        let c_stylesheet = CString::new(stylesheet).unwrap();
        unsafe { set_widget_stylesheet(self.ptr as *mut c_void, 15, c_stylesheet.as_ptr()) }
    }
}
