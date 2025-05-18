#include <QtWidgets/QGraphicsScene>
#include <QtWidgets/QGraphicsItem>
#include <QtGui/QBrush>
#include <QtGui/QColor>
#include <QtCore/QObject>

extern "C" {
    void* shawon_create_graphics_scene() {
        // Create a scene with a reasonable initial size
        QGraphicsScene* scene = new QGraphicsScene(0, 0, 800, 600);
        return scene;
    }
    
    void shawon_graphics_scene_add_item(void* scene, void* item) {
        if (scene && item) {
            QGraphicsScene* graphics_scene = static_cast<QGraphicsScene*>(scene);
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            
            graphics_scene->addItem(graphics_item);
        }
    }
    
    void shawon_graphics_scene_clear(void* scene) {
        if (scene) {
            QGraphicsScene* graphics_scene = static_cast<QGraphicsScene*>(scene);
            graphics_scene->clear();
        }
    }
    
    void shawon_graphics_scene_set_background_color(void* scene, const char* color) {
        if (scene && color) {
            QGraphicsScene* graphics_scene = static_cast<QGraphicsScene*>(scene);
            
            QString color_name = QString::fromUtf8(color);
            QColor background_color;
            
            if (color_name.startsWith("#")) {
                background_color = QColor(color_name);
            } else {
                background_color = QColor(color_name);
            }
            
            graphics_scene->setBackgroundBrush(QBrush(background_color));
        }
    }
}
