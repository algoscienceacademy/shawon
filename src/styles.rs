//! This module provides CSS styles for the application

/// Returns the main application style
pub fn get_app_style() -> &'static str {
    r#"
    QWidget {
        font-family: 'Segoe UI', Arial, sans-serif;
        font-size: 10pt;
    }

    QMainWindow {
        background-color: #f5f5f5;
    }

    QLabel {
        color: #333333;
    }

    QPushButton {
        background-color: #0078d7;
        color: white;
        border: none;
        border-radius: 4px;
        padding: 5px 15px;
        min-height: 20px;
    }

    QPushButton:hover {
        background-color: #00559b;
    }

    QPushButton:pressed {
        background-color: #003c6c;
    }

    QLineEdit {
        border: 1px solid #cccccc;
        border-radius: 4px;
        padding: 4px;
        background-color: white;
    }

    QLineEdit:focus {
        border: 1px solid #0078d7;
    }

    QTabWidget::pane {
        border: 1px solid #cccccc;
        border-radius: 4px;
        background-color: white;
    }

    QTabBar::tab {
        background-color: #f0f0f0;
        border: 1px solid #cccccc;
        border-bottom: none;
        border-top-left-radius: 4px;
        border-top-right-radius: 4px;
        padding: 6px 12px;
        margin-right: 2px;
    }

    QTabBar::tab:selected {
        background-color: white;
        border-bottom: 1px solid white;
    }

    QTabBar::tab:!selected {
        margin-top: 2px;
    }

    QTabBar::tab:hover:!selected {
        background-color: #e0e0e0;
    }

    QGroupBox {
        border: 1px solid #cccccc;
        border-radius: 4px;
        margin-top: 1.5ex;
        padding: 10px;
        background-color: white;
    }

    QGroupBox::title {
        subcontrol-origin: margin;
        subcontrol-position: top center;
        padding: 0 5px;
        color: #333333;
    }

    QCheckBox, QRadioButton {
        spacing: 8px;
    }

    QCheckBox::indicator, QRadioButton::indicator {
        width: 16px;
        height: 16px;
    }

    QSlider::groove:horizontal {
        border: 1px solid #cccccc;
        height: 8px;
        background: #f0f0f0;
        margin: 2px 0;
        border-radius: 4px;
    }

    QSlider::handle:horizontal {
        background: #0078d7;
        border: 1px solid #0078d7;
        width: 18px;
        margin: -2px 0;
        border-radius: 9px;
    }

    QSlider::handle:horizontal:hover {
        background: #00559b;
        border: 1px solid #00559b;
    }

    QComboBox {
        border: 1px solid #cccccc;
        border-radius: 4px;
        padding: 3px 18px 3px 5px;
        min-width: 6em;
        background-color: white;
    }

    QComboBox::drop-down {
        subcontrol-origin: padding;
        subcontrol-position: right;
        width: 15px;
        border-left: 1px solid #cccccc;
    }

    QProgressBar {
        border: 1px solid #cccccc;
        border-radius: 4px;
        text-align: center;
        background-color: #f0f0f0;
    }

    QProgressBar::chunk {
        background-color: #0078d7;
        width: 20px;
    }

    QListWidget, QTreeWidget, QTableWidget {
        border: 1px solid #cccccc;
        border-radius: 4px;
        background-color: white;
        alternate-background-color: #f9f9f9;
        selection-background-color: #0078d7;
        selection-color: white;
    }

    QHeaderView::section {
        background-color: #f0f0f0;
        border: 1px solid #cccccc;
        padding: 4px;
    }
    "#
}

/// Returns a dark theme style
pub fn get_dark_style() -> &'static str {
    r#"
    QWidget {
        font-family: 'Segoe UI', Arial, sans-serif;
        font-size: 10pt;
        background-color: #2b2b2b;
        color: #e0e0e0;
    }

    QMainWindow {
        background-color: #2b2b2b;
    }

    QLabel {
        color: #e0e0e0;
    }

    QPushButton {
        background-color: #0a84ff;
        color: white;
        border: none;
        border-radius: 4px;
        padding: 5px 15px;
        min-height: 20px;
    }

    QPushButton:hover {
        background-color: #0074e0;
    }

    QPushButton:pressed {
        background-color: #0064c0;
    }

    QLineEdit {
        border: 1px solid #555555;
        border-radius: 4px;
        padding: 4px;
        background-color: #3a3a3a;
        color: #e0e0e0;
    }

    QLineEdit:focus {
        border: 1px solid #0a84ff;
    }

    QTabWidget::pane {
        border: 1px solid #555555;
        border-radius: 4px;
        background-color: #2b2b2b;
    }

    QTabBar::tab {
        background-color: #3a3a3a;
        border: 1px solid #555555;
        border-bottom: none;
        border-top-left-radius: 4px;
        border-top-right-radius: 4px;
        padding: 6px 12px;
        margin-right: 2px;
    }

    QTabBar::tab:selected {
        background-color: #2b2b2b;
        border-bottom: 1px solid #2b2b2b;
    }

    QTabBar::tab:!selected {
        margin-top: 2px;
    }

    QTabBar::tab:hover:!selected {
        background-color: #444444;
    }

    QGroupBox {
        border: 1px solid #555555;
        border-radius: 4px;
        margin-top: 1.5ex;
        padding: 10px;
        background-color: #333333;
    }

    QGroupBox::title {
        subcontrol-origin: margin;
        subcontrol-position: top center;
        padding: 0 5px;
        color: #e0e0e0;
    }

    QCheckBox, QRadioButton {
        spacing: 8px;
    }

    QCheckBox::indicator, QRadioButton::indicator {
        width: 16px;
        height: 16px;
        background-color: #3a3a3a;
        border: 1px solid #555555;
    }

    QSlider::groove:horizontal {
        border: 1px solid #555555;
        height: 8px;
        background: #3a3a3a;
        margin: 2px 0;
        border-radius: 4px;
    }

    QSlider::handle:horizontal {
        background: #0a84ff;
        border: 1px solid #0a84ff;
        width: 18px;
        margin: -2px 0;
        border-radius: 9px;
    }

    QSlider::handle:horizontal:hover {
        background: #0074e0;
        border: 1px solid #0074e0;
    }

    QComboBox {
        border: 1px solid #555555;
        border-radius: 4px;
        padding: 3px 18px 3px 5px;
        min-width: 6em;
        background-color: #3a3a3a;
        color: #e0e0e0;
    }

    QComboBox::drop-down {
        subcontrol-origin: padding;
        subcontrol-position: right;
        width: 15px;
        border-left: 1px solid #555555;
    }

    QProgressBar {
        border: 1px solid #555555;
        border-radius: 4px;
        text-align: center;
        background-color: #3a3a3a;
        color: #e0e0e0;
    }

    QProgressBar::chunk {
        background-color: #0a84ff;
        width: 20px;
    }

    QListWidget, QTreeWidget, QTableWidget {
        border: 1px solid #555555;
        border-radius: 4px;
        background-color: #333333;
        alternate-background-color: #393939;
        selection-background-color: #0a84ff;
        selection-color: white;
    }

    QHeaderView::section {
        background-color: #3a3a3a;
        border: 1px solid #555555;
        padding: 4px;
        color: #e0e0e0;
    }
    "#
}
