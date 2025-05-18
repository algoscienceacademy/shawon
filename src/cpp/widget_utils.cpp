#include <QtWidgets/QWidget>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QCheckBox>
#include <QtWidgets/QRadioButton>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QListWidget>
#include <QtWidgets/QTreeWidget>
#include <QtWidgets/QTableWidget>
#include <QtWidgets/QTabWidget>
#include <QtWidgets/QProgressBar>
#include <QtWidgets/QSplitter>
#include <QtWidgets/QComboBox>
#include <QtWidgets/QSlider>
#include <QtWidgets/QGraphicsView>
#include <QtWidgets/QLayout>
#include <QtWidgets/QVBoxLayout>

extern "C" {
    void add_widget_to_window(void* window, void* widget, int widget_type) {
        if (!window || !widget) {
            return;
        }

        QWidget* parent_widget = static_cast<QWidget*>(window);
        QWidget* child_widget = nullptr;
        
        // Cast widget to appropriate type based on widget_type
        switch (widget_type) {
            case 1:  // Button
                child_widget = static_cast<QPushButton*>(widget);
                break;
            case 2:  // Label
                child_widget = static_cast<QLabel*>(widget);
                break;
            case 3:  // TextEntry (QLineEdit)
                child_widget = static_cast<QLineEdit*>(widget);
                break;
            case 4:  // CheckBox
                child_widget = static_cast<QCheckBox*>(widget);
                break;
            case 5:  // RadioButton
                child_widget = static_cast<QRadioButton*>(widget);
                break;
            case 6:  // GroupBox
                child_widget = static_cast<QGroupBox*>(widget);
                break;
            case 7:  // ListWidget
                child_widget = static_cast<QListWidget*>(widget);
                break;
            case 8:  // TreeWidget
                child_widget = static_cast<QTreeWidget*>(widget);
                break;
            case 9:  // TableWidget
                child_widget = static_cast<QTableWidget*>(widget);
                break;
            case 10: // Window (QWidget)
                child_widget = static_cast<QWidget*>(widget);
                break;
            case 11: // TabWidget
                child_widget = static_cast<QTabWidget*>(widget);
                break;
            case 12: // ProgressBar
                child_widget = static_cast<QProgressBar*>(widget);
                break;
            case 13: // Splitter
                child_widget = static_cast<QSplitter*>(widget);
                break;
            case 14: // ComboBox
                child_widget = static_cast<QComboBox*>(widget);
                break;
            case 15: // Slider
                child_widget = static_cast<QSlider*>(widget);
                break;
            case 20: // QGraphicsView
                child_widget = static_cast<QGraphicsView*>(widget);
                break;
            default:
                child_widget = static_cast<QWidget*>(widget);
        }
        
        if (child_widget) {
            child_widget->setParent(parent_widget);
            
            // If the parent widget doesn't have a layout, create one
            if (!parent_widget->layout()) {
                QVBoxLayout* layout = new QVBoxLayout(parent_widget);
                parent_widget->setLayout(layout);
            }
            
            // Add the widget to the parent's layout
            parent_widget->layout()->addWidget(child_widget);
        }
    }
}
