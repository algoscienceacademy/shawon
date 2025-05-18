#include <QtWidgets/QAbstractButton>
#include <QtWidgets/QPushButton>
#include <QtCore/QObject>

extern "C" {
    void* shawon_create_abstract_button() {
        // Since QAbstractButton is abstract, we use QPushButton as a concrete implementation
        return new QPushButton();
    }
    
    void shawon_abstract_button_set_text(void* button, const char* text) {
        if (button) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            btn->setText(QString::fromUtf8(text));
        }
    }
    
    const char* shawon_abstract_button_get_text(void* button) {
        if (button) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            // Note: This is a memory leak, in a real implementation you would need proper memory management
            return strdup(btn->text().toUtf8().constData());
        }
        return "";
    }
    
    void shawon_abstract_button_set_checkable(void* button, int checkable) {
        if (button) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            btn->setCheckable(checkable != 0);
        }
    }
    
    int shawon_abstract_button_is_checkable(void* button) {
        if (button) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            return btn->isCheckable() ? 1 : 0;
        }
        return 0;
    }
    
    void shawon_abstract_button_set_checked(void* button, int checked) {
        if (button) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            btn->setChecked(checked != 0);
        }
    }
    
    int shawon_abstract_button_is_checked(void* button) {
        if (button) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            return btn->isChecked() ? 1 : 0;
        }
        return 0;
    }
    
    // Callback for clicked signal
    typedef void (*ClickedCallback)(void* userData);
    
    // Structure to hold callback and user data
    struct ClickedData {
        ClickedCallback callback;
        void* userData;
    };
    
    void shawon_abstract_button_on_clicked(void* button, ClickedCallback callback, void* userData) {
        if (button && callback) {
            QAbstractButton* btn = static_cast<QAbstractButton*>(button);
            
            // Create a data structure to hold our callback
            ClickedData* data = new ClickedData{callback, userData};
            
            // Connect using a lambda that invokes our callback
            QObject::connect(btn, &QAbstractButton::clicked, [data]() {
                data->callback(data->userData);
            });
        }
    }
}
