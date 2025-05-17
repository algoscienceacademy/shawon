#include <QtWidgets/QApplication>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QLabel>
#include <QtWidgets/QLineEdit>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QGridLayout>
#include <QtWidgets/QWidget>
#include <QtWidgets/QAbstractButton>
#include <QtWidgets/QCheckBox>
#include <QtWidgets/QRadioButton>
#include <QtWidgets/QToolButton>
#include <QtWidgets/QGroupBox>
#include <QtWidgets/QComboBox>
#include <QtWidgets/QSlider>
#include <QtWidgets/QSpinBox>
#include <QtWidgets/QTabWidget>
#include <QtWidgets/QProgressBar>
#include <QtWidgets/QFileDialog>
#include <QtWidgets/QMessageBox>
#include <QtWidgets/QListWidget>
#include <QtWidgets/QTreeWidget>
#include <QtWidgets/QTableWidget>
#include <QtWidgets/QSplitter>
#include <string>
#include <memory>

extern "C" {
    // Application
    struct RustQApplication {
        int argc;
        char** argv;
        QApplication* app;
    };

    RustQApplication* create_application(int argc, char** argv) {
        auto rustApp = new RustQApplication;
        rustApp->argc = argc;
        rustApp->argv = argv;
        rustApp->app = new QApplication(rustApp->argc, rustApp->argv);
        return rustApp;
    }

    int exec_application(RustQApplication* app) {
        return app->app->exec();
    }

    void destroy_application(RustQApplication* app) {
        delete app->app;
        delete app;
    }

    // Application stylesheet support
    void set_application_stylesheet(RustQApplication* app, const char* stylesheet) {
        app->app->setStyleSheet(QString::fromUtf8(stylesheet));
    }

    // Window
    struct RustQWidget {
        QWidget* widget;
        QVBoxLayout* layout;
    };

    RustQWidget* create_window(const char* title, int width, int height) {
        auto rustWidget = new RustQWidget;
        rustWidget->widget = new QWidget();
        rustWidget->widget->setWindowTitle(QString::fromUtf8(title));
        rustWidget->widget->resize(width, height);
        
        rustWidget->layout = new QVBoxLayout(rustWidget->widget);
        rustWidget->widget->setLayout(rustWidget->layout);
        
        return rustWidget;
    }

    void show_window(RustQWidget* window) {
        window->widget->show();
    }

    void destroy_window(RustQWidget* window) {
        delete window->widget; // This also deletes the layout
        delete window;
    }

    // Button
    struct RustQButton {
        QPushButton* button;
        void (*callback)(void*);
        void* user_data;
    };

    RustQButton* create_button(const char* text) {
        auto rustButton = new RustQButton;
        rustButton->button = new QPushButton(QString::fromUtf8(text));
        rustButton->callback = nullptr;
        rustButton->user_data = nullptr;
        return rustButton;
    }

    void set_button_callback(RustQButton* button, void (*callback)(void*), void* user_data) {
        button->callback = callback;
        button->user_data = user_data;
        
        button->button->disconnect();
        QObject::connect(button->button, &QPushButton::clicked, [button]() {
            if (button->callback) {
                button->callback(button->user_data);
            }
        });
    }

    void add_button_to_window(RustQWidget* window, RustQButton* button) {
        window->layout->addWidget(button->button);
    }

    void destroy_button(RustQButton* button) {
        delete button->button;
        delete button;
    }

    // Label
    struct RustQLabel {
        QLabel* label;
    };

    RustQLabel* create_label(const char* text) {
        auto rustLabel = new RustQLabel;
        rustLabel->label = new QLabel(QString::fromUtf8(text));
        return rustLabel;
    }

    void set_label_text(RustQLabel* label, const char* text) {
        label->label->setText(QString::fromUtf8(text));
    }

    void add_label_to_window(RustQWidget* window, RustQLabel* label) {
        window->layout->addWidget(label->label);
    }

    void destroy_label(RustQLabel* label) {
        delete label->label;
        delete label;
    }

    // Text Entry
    struct RustQLineEdit {
        QLineEdit* line_edit;
    };

    RustQLineEdit* create_text_entry() {
        auto rustLineEdit = new RustQLineEdit;
        rustLineEdit->line_edit = new QLineEdit();
        return rustLineEdit;
    }

    void set_text_entry_text(RustQLineEdit* entry, const char* text) {
        entry->line_edit->setText(QString::fromUtf8(text));
    }

    const char* get_text_entry_text(RustQLineEdit* entry) {
        QByteArray bytes = entry->line_edit->text().toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    void add_text_entry_to_window(RustQWidget* window, RustQLineEdit* entry) {
        window->layout->addWidget(entry->line_edit);
    }

    void destroy_text_entry(RustQLineEdit* entry) {
        delete entry->line_edit;
        delete entry;
    }

    void free_string(const char* str) {
        free((void*)str);
    }

    // QAbstractButton implementation
    struct RustQAbstractButton {
        QAbstractButton* button;
        void (*clicked_callback)(void*);
        void (*toggled_callback)(void*, int);
        void* clicked_user_data;
        void* toggled_user_data;
    };

    void set_abstract_button_text(RustQAbstractButton* button, const char* text) {
        button->button->setText(QString::fromUtf8(text));
    }

    const char* get_abstract_button_text(RustQAbstractButton* button) {
        QByteArray bytes = button->button->text().toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    void set_abstract_button_icon(RustQAbstractButton* button, const char* path) {
        button->button->setIcon(QIcon(QString::fromUtf8(path)));
    }

    void set_abstract_button_checkable(RustQAbstractButton* button, int checkable) {
        button->button->setCheckable(checkable != 0);
    }

    int is_abstract_button_checked(RustQAbstractButton* button) {
        return button->button->isChecked() ? 1 : 0;
    }

    void set_abstract_button_checked(RustQAbstractButton* button, int checked) {
        button->button->setChecked(checked != 0);
    }

    void set_abstract_button_clicked_callback(RustQAbstractButton* button, void (*callback)(void*), void* user_data) {
        button->clicked_callback = callback;
        button->clicked_user_data = user_data;
        
        QObject::connect(button->button, &QAbstractButton::clicked, [button]() {
            if (button->clicked_callback) {
                button->clicked_callback(button->clicked_user_data);
            }
        });
    }

    void set_abstract_button_toggled_callback(RustQAbstractButton* button, void (*callback)(void*, int), void* user_data) {
        button->toggled_callback = callback;
        button->toggled_user_data = user_data;
        
        QObject::connect(button->button, &QAbstractButton::toggled, [button](bool checked) {
            if (button->toggled_callback) {
                button->toggled_callback(button->toggled_user_data, checked ? 1 : 0);
            }
        });
    }

    // QCheckBox implementation
    struct RustQCheckBox {
        QCheckBox* checkbox;
        RustQAbstractButton abstractButton;
    };

    RustQCheckBox* create_checkbox(const char* text) {
        auto rustCheckBox = new RustQCheckBox;
        rustCheckBox->checkbox = new QCheckBox(QString::fromUtf8(text));
        rustCheckBox->abstractButton.button = rustCheckBox->checkbox;
        rustCheckBox->abstractButton.clicked_callback = nullptr;
        rustCheckBox->abstractButton.toggled_callback = nullptr;
        rustCheckBox->abstractButton.clicked_user_data = nullptr;
        rustCheckBox->abstractButton.toggled_user_data = nullptr;
        return rustCheckBox;
    }

    RustQAbstractButton* checkbox_as_abstract_button(RustQCheckBox* checkbox) {
        return &checkbox->abstractButton;
    }

    void add_checkbox_to_window(RustQWidget* window, RustQCheckBox* checkbox) {
        window->layout->addWidget(checkbox->checkbox);
    }

    void destroy_checkbox(RustQCheckBox* checkbox) {
        delete checkbox;
    }

    // QRadioButton implementation
    struct RustQRadioButton {
        QRadioButton* radio;
        RustQAbstractButton abstractButton;
    };

    RustQRadioButton* create_radio_button(const char* text) {
        auto rustRadioButton = new RustQRadioButton;
        rustRadioButton->radio = new QRadioButton(QString::fromUtf8(text));
        rustRadioButton->abstractButton.button = rustRadioButton->radio;
        rustRadioButton->abstractButton.clicked_callback = nullptr;
        rustRadioButton->abstractButton.toggled_callback = nullptr;
        rustRadioButton->abstractButton.clicked_user_data = nullptr;
        rustRadioButton->abstractButton.toggled_user_data = nullptr;
        return rustRadioButton;
    }

    RustQAbstractButton* radio_button_as_abstract_button(RustQRadioButton* radio) {
        return &radio->abstractButton;
    }

    void add_radio_button_to_window(RustQWidget* window, RustQRadioButton* radio) {
        window->layout->addWidget(radio->radio);
    }

    void destroy_radio_button(RustQRadioButton* radio) {
        delete radio;
    }

    // QGroupBox implementation
    struct RustQGroupBox {
        QGroupBox* group;
        QVBoxLayout* layout;
    };

    RustQGroupBox* create_group_box(const char* title) {
        auto rustGroupBox = new RustQGroupBox;
        rustGroupBox->group = new QGroupBox(QString::fromUtf8(title));
        rustGroupBox->layout = new QVBoxLayout(rustGroupBox->group);
        rustGroupBox->group->setLayout(rustGroupBox->layout);
        return rustGroupBox;
    }

    void add_widget_to_group_box(RustQGroupBox* group, void* widget, int widget_type) {
        QWidget* qwidget = nullptr;
        
        switch (widget_type) {
            case 1: 
                qwidget = ((RustQButton*)widget)->button;
                break;
            case 2: 
                qwidget = ((RustQLabel*)widget)->label;
                break;
            case 3: 
                qwidget = ((RustQLineEdit*)widget)->line_edit;
                break;
            case 4: 
                qwidget = ((RustQCheckBox*)widget)->checkbox;
                break;
            case 5: 
                qwidget = ((RustQRadioButton*)widget)->radio;
                break;
            default:
                return;
        }
        
        if (qwidget) {
            group->layout->addWidget(qwidget);
        }
    }

    void add_group_box_to_window(RustQWidget* window, RustQGroupBox* group) {
        window->layout->addWidget(group->group);
    }

    void destroy_group_box(RustQGroupBox* group) {
        delete group;
    }

    // Layout implementations
    struct RustQHBoxLayout {
        QHBoxLayout* layout;
    };

    RustQHBoxLayout* create_hbox_layout() {
        auto rustLayout = new RustQHBoxLayout;
        rustLayout->layout = new QHBoxLayout();
        return rustLayout;
    }

    void add_widget_to_hbox_layout(RustQHBoxLayout* layout, void* widget, int widget_type) {
        QWidget* qwidget = nullptr;
        
        switch (widget_type) {
            case 1: 
                qwidget = ((RustQButton*)widget)->button;
                break;
            case 2: 
                qwidget = ((RustQLabel*)widget)->label;
                break;
            case 3: 
                qwidget = ((RustQLineEdit*)widget)->line_edit;
                break;
            case 4: 
                qwidget = ((RustQCheckBox*)widget)->checkbox;
                break;
            case 5: 
                qwidget = ((RustQRadioButton*)widget)->radio;
                break;
            case 6: 
                qwidget = ((RustQGroupBox*)widget)->group;
                break;
            default:
                return;
        }
        
        if (qwidget) {
            layout->layout->addWidget(qwidget);
        }
    }

    void add_layout_to_window(RustQWidget* window, RustQHBoxLayout* layout) {
        window->layout->addLayout(layout->layout);
    }

    void destroy_hbox_layout(RustQHBoxLayout* layout) {
        delete layout;
    }

    // QSlider implementation
    struct RustQSlider {
        QSlider* slider;
        void (*value_changed_callback)(void*, int);
        void* user_data;
    };

    RustQSlider* create_slider(int orientation) {
        auto rustSlider = new RustQSlider;
        rustSlider->slider = new QSlider(orientation == 0 ? Qt::Horizontal : Qt::Vertical);
        rustSlider->value_changed_callback = nullptr;
        rustSlider->user_data = nullptr;
        return rustSlider;
    }

    void set_slider_range(RustQSlider* slider, int min, int max) {
        slider->slider->setRange(min, max);
    }

    void set_slider_value(RustQSlider* slider, int value) {
        slider->slider->setValue(value);
    }

    int get_slider_value(RustQSlider* slider) {
        return slider->slider->value();
    }

    void set_slider_value_changed_callback(RustQSlider* slider, void (*callback)(void*, int), void* user_data) {
        slider->value_changed_callback = callback;
        slider->user_data = user_data;
        
        QObject::connect(slider->slider, &QSlider::valueChanged, [slider](int value) {
            if (slider->value_changed_callback) {
                slider->value_changed_callback(slider->user_data, value);
            }
        });
    }

    void add_slider_to_window(RustQWidget* window, RustQSlider* slider) {
        window->layout->addWidget(slider->slider);
    }

    void destroy_slider(RustQSlider* slider) {
        delete slider;
    }

    // QComboBox implementation
    struct RustQComboBox {
        QComboBox* combo;
        void (*current_index_changed_callback)(void*, int);
        void* user_data;
    };

    RustQComboBox* create_combo_box() {
        auto rustComboBox = new RustQComboBox;
        rustComboBox->combo = new QComboBox();
        rustComboBox->current_index_changed_callback = nullptr;
        rustComboBox->user_data = nullptr;
        return rustComboBox;
    }

    void add_combo_box_item(RustQComboBox* combo, const char* text) {
        combo->combo->addItem(QString::fromUtf8(text));
    }

    void set_combo_box_current_index(RustQComboBox* combo, int index) {
        combo->combo->setCurrentIndex(index);
    }

    int get_combo_box_current_index(RustQComboBox* combo) {
        return combo->combo->currentIndex();
    }

    const char* get_combo_box_current_text(RustQComboBox* combo) {
        QByteArray bytes = combo->combo->currentText().toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    void set_combo_box_index_changed_callback(RustQComboBox* combo, void (*callback)(void*, int), void* user_data) {
        combo->current_index_changed_callback = callback;
        combo->user_data = user_data;
        
        QObject::connect(combo->combo, QOverload<int>::of(&QComboBox::currentIndexChanged), [combo](int index) {
            if (combo->current_index_changed_callback) {
                combo->current_index_changed_callback(combo->user_data, index);
            }
        });
    }

    void add_combo_box_to_window(RustQWidget* window, RustQComboBox* combo) {
        window->layout->addWidget(combo->combo);
    }

    void destroy_combo_box(RustQComboBox* combo) {
        delete combo;
    }

    // QTabWidget implementation
    struct RustQTabWidget {
        QTabWidget* tab_widget;
    };

    RustQTabWidget* create_tab_widget() {
        auto rustTabWidget = new RustQTabWidget;
        rustTabWidget->tab_widget = new QTabWidget();
        return rustTabWidget;
    }

    void add_tab_to_tab_widget(RustQTabWidget* tabs, RustQWidget* widget, const char* label) {
        tabs->tab_widget->addTab(widget->widget, QString::fromUtf8(label));
    }

    int get_current_tab_index(RustQTabWidget* tabs) {
        return tabs->tab_widget->currentIndex();
    }

    void set_current_tab_index(RustQTabWidget* tabs, int index) {
        tabs->tab_widget->setCurrentIndex(index);
    }

    void set_tab_changed_callback(RustQTabWidget* tabs, void (*callback)(void*, int), void* user_data) {
        QObject::connect(tabs->tab_widget, &QTabWidget::currentChanged, [tabs, callback, user_data](int index) {
            if (callback) {
                callback(user_data, index);
            }
        });
    }

    void add_tab_widget_to_window(RustQWidget* window, RustQTabWidget* tabs) {
        window->layout->addWidget(tabs->tab_widget);
    }

    void destroy_tab_widget(RustQTabWidget* tabs) {
        // The widget will be destroyed with its parent
        delete tabs;
    }

    // QProgressBar implementation
    struct RustQProgressBar {
        QProgressBar* progress_bar;
    };

    RustQProgressBar* create_progress_bar() {
        auto rustProgressBar = new RustQProgressBar;
        rustProgressBar->progress_bar = new QProgressBar();
        return rustProgressBar;
    }

    void set_progress_bar_range(RustQProgressBar* bar, int min, int max) {
        bar->progress_bar->setRange(min, max);
    }

    void set_progress_bar_value(RustQProgressBar* bar, int value) {
        bar->progress_bar->setValue(value);
    }

    int get_progress_bar_value(RustQProgressBar* bar) {
        return bar->progress_bar->value();
    }

    void set_progress_bar_text_visible(RustQProgressBar* bar, int visible) {
        bar->progress_bar->setTextVisible(visible != 0);
    }

    void add_progress_bar_to_window(RustQWidget* window, RustQProgressBar* bar) {
        window->layout->addWidget(bar->progress_bar);
    }

    void destroy_progress_bar(RustQProgressBar* bar) {
        // The widget will be destroyed with its parent
        delete bar;
    }

    // QFileDialog implementation
    const char* show_open_file_dialog(RustQWidget* parent, const char* caption, const char* dir, const char* filter) {
        QString fileName = QFileDialog::getOpenFileName(
            parent->widget,
            QString::fromUtf8(caption),
            QString::fromUtf8(dir),
            QString::fromUtf8(filter)
        );
        
        if (fileName.isEmpty()) {
            return nullptr;
        }
        
        QByteArray bytes = fileName.toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    const char* show_save_file_dialog(RustQWidget* parent, const char* caption, const char* dir, const char* filter) {
        QString fileName = QFileDialog::getSaveFileName(
            parent->widget,
            QString::fromUtf8(caption),
            QString::fromUtf8(dir),
            QString::fromUtf8(filter)
        );
        
        if (fileName.isEmpty()) {
            return nullptr;
        }
        
        QByteArray bytes = fileName.toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    void show_message_box(RustQWidget* parent, int icon, const char* title, const char* text) {
        QMessageBox::Icon qicon;
        switch (icon) {
            case 0: qicon = QMessageBox::Information; break;
            case 1: qicon = QMessageBox::Warning; break;
            case 2: qicon = QMessageBox::Critical; break;
            case 3: qicon = QMessageBox::Question; break;
            default: qicon = QMessageBox::NoIcon;
        }
        
        QMessageBox msgBox(qicon, QString::fromUtf8(title), QString::fromUtf8(text), QMessageBox::Ok, parent->widget);
        msgBox.exec();
    }

    // QListWidget implementation
    struct RustQListWidget {
        QListWidget* list_widget;
        void (*item_clicked_callback)(void*, int);
        void* user_data;
    };

    RustQListWidget* create_list_widget() {
        auto rustListWidget = new RustQListWidget;
        rustListWidget->list_widget = new QListWidget();
        rustListWidget->item_clicked_callback = nullptr;
        rustListWidget->user_data = nullptr;
        return rustListWidget;
    }

    void add_list_item(RustQListWidget* list, const char* text) {
        list->list_widget->addItem(QString::fromUtf8(text));
    }

    void clear_list(RustQListWidget* list) {
        list->list_widget->clear();
    }

    int get_selected_list_row(RustQListWidget* list) {
        QList<QListWidgetItem*> selected = list->list_widget->selectedItems();
        if (selected.isEmpty()) {
            return -1;
        }
        return list->list_widget->row(selected.first());
    }

    const char* get_list_item_text(RustQListWidget* list, int row) {
        QListWidgetItem* item = list->list_widget->item(row);
        if (!item) {
            return nullptr;
        }
        
        QByteArray bytes = item->text().toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    void set_list_item_clicked_callback(RustQListWidget* list, void (*callback)(void*, int), void* user_data) {
        list->item_clicked_callback = callback;
        list->user_data = user_data;
        
        QObject::connect(list->list_widget, &QListWidget::itemClicked, [list](QListWidgetItem* item) {
            if (list->item_clicked_callback) {
                int row = list->list_widget->row(item);
                list->item_clicked_callback(list->user_data, row);
            }
        });
    }

    void add_list_widget_to_window(RustQWidget* window, RustQListWidget* list) {
        window->layout->addWidget(list->list_widget);
    }

    void destroy_list_widget(RustQListWidget* list) {
        // The widget will be destroyed with its parent
        delete list;
    }

    // QTreeWidget implementation
    struct RustQTreeWidget {
        QTreeWidget* tree_widget;
        void (*item_clicked_callback)(void*, int, int);
        void* user_data;
    };

    RustQTreeWidget* create_tree_widget() {
        auto rustTreeWidget = new RustQTreeWidget;
        rustTreeWidget->tree_widget = new QTreeWidget();
        rustTreeWidget->item_clicked_callback = nullptr;
        rustTreeWidget->user_data = nullptr;
        return rustTreeWidget;
    }

    void set_tree_headers(RustQTreeWidget* tree, const char* headers[], int count) {
        QStringList headerList;
        for (int i = 0; i < count; i++) {
            headerList << QString::fromUtf8(headers[i]);
        }
        tree->tree_widget->setHeaderLabels(headerList);
    }

    int add_tree_top_item(RustQTreeWidget* tree, const char* text) {
        QTreeWidgetItem* item = new QTreeWidgetItem();
        item->setText(0, QString::fromUtf8(text));
        tree->tree_widget->addTopLevelItem(item);
        return tree->tree_widget->indexOfTopLevelItem(item);
    }

    int add_tree_child_item(RustQTreeWidget* tree, int parent_index, const char* text) {
        QTreeWidgetItem* parent = tree->tree_widget->topLevelItem(parent_index);
        if (!parent) {
            return -1;
        }
        
        QTreeWidgetItem* item = new QTreeWidgetItem();
        item->setText(0, QString::fromUtf8(text));
        parent->addChild(item);
        return parent->indexOfChild(item);
    }

    void set_tree_item_text(RustQTreeWidget* tree, int parent_index, int child_index, int column, const char* text) {
        QTreeWidgetItem* item;
        
        if (child_index >= 0) {
            QTreeWidgetItem* parent = tree->tree_widget->topLevelItem(parent_index);
            if (!parent) {
                return;
            }
            item = parent->child(child_index);
        } else {
            item = tree->tree_widget->topLevelItem(parent_index);
        }
        
        if (item) {
            item->setText(column, QString::fromUtf8(text));
        }
    }

    void expand_tree_item(RustQTreeWidget* tree, int index) {
        QTreeWidgetItem* item = tree->tree_widget->topLevelItem(index);
        if (item) {
            item->setExpanded(true);
        }
    }

    void set_tree_item_clicked_callback(RustQTreeWidget* tree, void (*callback)(void*, int, int), void* user_data) {
        tree->item_clicked_callback = callback;
        tree->user_data = user_data;
        
        QObject::connect(tree->tree_widget, &QTreeWidget::itemClicked, [tree](QTreeWidgetItem* item, int column) {
            if (tree->item_clicked_callback) {
                QTreeWidgetItem* parent = item->parent();
                int parentIndex = -1;
                int childIndex = -1;
                
                if (parent) {
                    parentIndex = tree->tree_widget->indexOfTopLevelItem(parent);
                    childIndex = parent->indexOfChild(item);
                } else {
                    parentIndex = tree->tree_widget->indexOfTopLevelItem(item);
                }
                
                tree->item_clicked_callback(tree->user_data, parentIndex, childIndex);
            }
        });
    }

    void add_tree_widget_to_window(RustQWidget* window, RustQTreeWidget* tree) {
        window->layout->addWidget(tree->tree_widget);
    }

    void destroy_tree_widget(RustQTreeWidget* tree) {
        // The widget will be destroyed with its parent
        delete tree;
    }

    // QTableWidget implementation
    struct RustQTableWidget {
        QTableWidget* table_widget;
        void (*cell_clicked_callback)(void*, int, int);
        void* user_data;
    };

    RustQTableWidget* create_table_widget(int rows, int columns) {
        auto rustTableWidget = new RustQTableWidget;
        rustTableWidget->table_widget = new QTableWidget(rows, columns);
        rustTableWidget->cell_clicked_callback = nullptr;
        rustTableWidget->user_data = nullptr;
        return rustTableWidget;
    }

    void set_table_headers(RustQTableWidget* table, const char* headers[], int count, int orientation) {
        QStringList headerList;
        for (int i = 0; i < count; i++) {
            headerList << QString::fromUtf8(headers[i]);
        }
        
        if (orientation == 0) {
            // Horizontal headers
            table->table_widget->setHorizontalHeaderLabels(headerList);
        } else {
            // Vertical headers
            table->table_widget->setVerticalHeaderLabels(headerList);
        }
    }

    void set_table_cell_text(RustQTableWidget* table, int row, int column, const char* text) {
        QTableWidgetItem* item = new QTableWidgetItem(QString::fromUtf8(text));
        table->table_widget->setItem(row, column, item);
    }

    const char* get_table_cell_text(RustQTableWidget* table, int row, int column) {
        QTableWidgetItem* item = table->table_widget->item(row, column);
        if (!item) {
            return nullptr;
        }
        
        QByteArray bytes = item->text().toUtf8();
        char* result = (char*)malloc(bytes.size() + 1);
        strcpy(result, bytes.constData());
        return result;
    }

    void set_table_cell_clicked_callback(RustQTableWidget* table, void (*callback)(void*, int, int), void* user_data) {
        table->cell_clicked_callback = callback;
        table->user_data = user_data;
        
        QObject::connect(table->table_widget, &QTableWidget::cellClicked, [table](int row, int column) {
            if (table->cell_clicked_callback) {
                table->cell_clicked_callback(table->user_data, row, column);
            }
        });
    }

    void resize_table_columns_to_contents(RustQTableWidget* table) {
        table->table_widget->resizeColumnsToContents();
    }

    void add_table_widget_to_window(RustQWidget* window, RustQTableWidget* table) {
        window->layout->addWidget(table->table_widget);
    }

    void destroy_table_widget(RustQTableWidget* table) {
        // The widget will be destroyed with its parent
        delete table;
    }

    // QSplitter implementation
    struct RustQSplitter {
        QSplitter* splitter;
    };

    RustQSplitter* create_splitter(int orientation) {
        auto rustSplitter = new RustQSplitter;
        rustSplitter->splitter = new QSplitter(orientation == 0 ? Qt::Horizontal : Qt::Vertical);
        return rustSplitter;
    }

    void add_widget_to_splitter(RustQSplitter* splitter, void* widget, int widget_type) {
        QWidget* qwidget = nullptr;
        
        // Map widget type to actual widget
        switch (widget_type) {
            case 1: // Button
                qwidget = ((RustQButton*)widget)->button;
                break;
            case 2: // Label
                qwidget = ((RustQLabel*)widget)->label;
                break;
            case 3: // LineEdit
                qwidget = ((RustQLineEdit*)widget)->line_edit;
                break;
            case 4: // CheckBox
                qwidget = ((RustQCheckBox*)widget)->checkbox;
                break;
            case 5: // RadioButton
                qwidget = ((RustQRadioButton*)widget)->radio;
                break;
            case 6: // GroupBox
                qwidget = ((RustQGroupBox*)widget)->group;
                break;
            case 7: // ListWidget
                qwidget = ((RustQListWidget*)widget)->list_widget;
                break;
            case 8: // TreeWidget
                qwidget = ((RustQTreeWidget*)widget)->tree_widget;
                break;
            case 9: // TableWidget
                qwidget = ((RustQTableWidget*)widget)->table_widget;
                break;
            case 10: // Window
                qwidget = ((RustQWidget*)widget)->widget;
                break;
            default:
                return;
        }
        
        if (qwidget) {
            splitter->splitter->addWidget(qwidget);
        }
    }

    void set_splitter_sizes(RustQSplitter* splitter, int sizes[], int count) {
        QList<int> sizeList;
        for (int i = 0; i < count; i++) {
            sizeList << sizes[i];
        }
        splitter->splitter->setSizes(sizeList);
    }

    void add_splitter_to_window(RustQWidget* window, RustQSplitter* splitter) {
        window->layout->addWidget(splitter->splitter);
    }

    void destroy_splitter(RustQSplitter* splitter) {
        // The widget will be destroyed with its parent
        delete splitter;
    }

    // Widget-specific stylesheet support
    void set_widget_stylesheet(void* widget, int widget_type, const char* stylesheet) {
        QWidget* qwidget = nullptr;
        
        // Map widget type to actual widget
        switch (widget_type) {
            case 1: // Button
                qwidget = ((RustQButton*)widget)->button;
                break;
            case 2: // Label
                qwidget = ((RustQLabel*)widget)->label;
                break;
            case 3: // LineEdit
                qwidget = ((RustQLineEdit*)widget)->line_edit;
                break;
            case 4: // CheckBox
                qwidget = ((RustQCheckBox*)widget)->checkbox;
                break;
            case 5: // RadioButton
                qwidget = ((RustQRadioButton*)widget)->radio;
                break;
            case 6: // GroupBox
                qwidget = ((RustQGroupBox*)widget)->group;
                break;
            case 7: // ListWidget
                qwidget = ((RustQListWidget*)widget)->list_widget;
                break;
            case 8: // TreeWidget
                qwidget = ((RustQTreeWidget*)widget)->tree_widget;
                break;
            case 9: // TableWidget
                qwidget = ((RustQTableWidget*)widget)->table_widget;
                break;
            case 10: // Window
                qwidget = ((RustQWidget*)widget)->widget;
                break;
            case 11: // TabWidget
                qwidget = ((RustQTabWidget*)widget)->tab_widget;
                break;
            case 12: // ProgressBar
                qwidget = ((RustQProgressBar*)widget)->progress_bar;
                break;
            case 13: // Splitter
                qwidget = ((RustQSplitter*)widget)->splitter;
                break;
            case 14: // ComboBox
                qwidget = ((RustQComboBox*)widget)->combo;
                break;
            case 15: // Slider
                qwidget = ((RustQSlider*)widget)->slider;
                break;
            default:
                return;
        }
        
        if (qwidget) {
            qwidget->setStyleSheet(QString::fromUtf8(stylesheet));
        }
    }
}
