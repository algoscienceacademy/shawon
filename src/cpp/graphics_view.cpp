#include <QtWidgets/QGraphicsView>
#include <QtWidgets/QGraphicsScene>
#include <QtCore/QObject>

extern "C" {
    void* shawon_create_graphics_view(void* scene) {
        QGraphicsView* view = new QGraphicsView();
        
        if (scene) {
            QGraphicsScene* graphics_scene = static_cast<QGraphicsScene*>(scene);
            view->setScene(graphics_scene);
        }
        
        return view;
    }
    
    void shawon_graphics_view_set_scene(void* view, void* scene) {
        if (view && scene) {
            QGraphicsView* graphics_view = static_cast<QGraphicsView*>(view);
            QGraphicsScene* graphics_scene = static_cast<QGraphicsScene*>(scene);
            
            graphics_view->setScene(graphics_scene);
        }
    }
    
    void shawon_graphics_view_resize(void* view, int width, int height) {
        if (view) {
            QGraphicsView* graphics_view = static_cast<QGraphicsView*>(view);
            graphics_view->resize(width, height);
        }
    }
    
    void shawon_graphics_view_fit_in_view(void* view) {
        if (view) {
            QGraphicsView* graphics_view = static_cast<QGraphicsView*>(view);
            graphics_view->fitInView(graphics_view->scene()->sceneRect(), Qt::KeepAspectRatio);
        }
    }
}
