#include <QtWidgets/QGraphicsScene>
#include <QtWidgets/QGraphicsRectItem>
#include <QtWidgets/QGraphicsEllipseItem>
#include <QtWidgets/QGraphicsLineItem>
#include <QtWidgets/QGraphicsPathItem>
#include <QtGui/QPen>
#include <QtGui/QBrush>
#include <QtGui/QPainterPath>
#include <QtCore/QObject>
#include <cstring>

extern "C" {
    // Helper function to create a QColor from string
    QColor string_to_color(const char* color_str) {
        if (!color_str || !*color_str) {
            return QColor(Qt::black);
        }
        
        QString color_name = QString::fromUtf8(color_str);
        if (color_name.startsWith("#")) {
            return QColor(color_name);
        } else {
            return QColor(color_name);
        }
    }
    
    // Create different types of graphics shape items
    void* shawon_create_graphics_rect_item(double x, double y, double width, double height) {
        QGraphicsRectItem* item = new QGraphicsRectItem(x, y, width, height);
        return item;
    }
    
    void* shawon_create_graphics_ellipse_item(double x, double y, double width, double height) {
        QGraphicsEllipseItem* item = new QGraphicsEllipseItem(x, y, width, height);
        return item;
    }
    
    void* shawon_create_graphics_line_item(double x1, double y1, double x2, double y2) {
        QGraphicsLineItem* item = new QGraphicsLineItem(x1, y1, x2, y2);
        return item;
    }
    
    void* shawon_create_graphics_path_item() {
        QGraphicsPathItem* item = new QGraphicsPathItem();
        return item;
    }
    
    // Common operations for all shape items
    void shawon_graphics_shape_item_set_pos(void* item, double x, double y) {
        if (item) {
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            graphics_item->setPos(x, y);
        }
    }
    
    void shawon_graphics_shape_item_move_by(void* item, double dx, double dy) {
        if (item) {
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            graphics_item->moveBy(dx, dy);
        }
    }
    
    void shawon_graphics_shape_item_set_rotation(void* item, double angle) {
        if (item) {
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            graphics_item->setRotation(angle);
        }
    }
    
    void shawon_graphics_shape_item_set_scale(void* item, double sx, double sy) {
        if (item) {
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            graphics_item->setScale(sx);
            // Note: QGraphicsItem::setScale only takes one scale factor
            // We ignore the sy parameter as Qt doesn't support separate x/y scaling
            // without using a transform
        }
    }
    
    void shawon_graphics_shape_item_set_visible(void* item, int visible) {
        if (item) {
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            graphics_item->setVisible(visible != 0);
        }
    }
    
    void shawon_graphics_shape_item_set_z_value(void* item, double z) {
        if (item) {
            QGraphicsItem* graphics_item = static_cast<QGraphicsItem*>(item);
            graphics_item->setZValue(z);
        }
    }
    
    void shawon_graphics_shape_item_set_pen(void* item, const char* color, double width, int style) {
        if (item) {
            QAbstractGraphicsShapeItem* shape_item = static_cast<QAbstractGraphicsShapeItem*>(item);
            
            QPen pen;
            pen.setColor(string_to_color(color));
            pen.setWidthF(width);
            
            // Set the pen style
            switch (style) {
                case 0:
                    pen.setStyle(Qt::NoPen);
                    break;
                case 1:
                    pen.setStyle(Qt::SolidLine);
                    break;
                case 2:
                    pen.setStyle(Qt::DashLine);
                    break;
                case 3:
                    pen.setStyle(Qt::DotLine);
                    break;
                case 4:
                    pen.setStyle(Qt::DashDotLine);
                    break;
                case 5:
                    pen.setStyle(Qt::DashDotDotLine);
                    break;
                default:
                    pen.setStyle(Qt::SolidLine);
            }
            
            shape_item->setPen(pen);
        }
    }
    
    void shawon_graphics_shape_item_set_brush(void* item, const char* color, int style) {
        if (item) {
            QAbstractGraphicsShapeItem* shape_item = static_cast<QAbstractGraphicsShapeItem*>(item);
            
            QBrush brush;
            brush.setColor(string_to_color(color));
            
            // Set the brush style
            switch (style) {
                case 0:
                    brush.setStyle(Qt::NoBrush);
                    break;
                case 1:
                    brush.setStyle(Qt::SolidPattern);
                    break;
                case 2:
                    brush.setStyle(Qt::HorPattern);  // Correct enum name
                    break;
                case 3:
                    brush.setStyle(Qt::VerPattern);  // Correct enum name
                    break;
                case 4:
                    brush.setStyle(Qt::CrossPattern);
                    break;
                case 5:
                    brush.setStyle(Qt::DiagCrossPattern);
                    break;
                default:
                    brush.setStyle(Qt::SolidPattern);
            }
            
            shape_item->setBrush(brush);
        }
    }
    
    // Path item specific operations
    void shawon_graphics_path_item_move_to(void* item, double x, double y) {
        if (item) {
            QGraphicsPathItem* path_item = static_cast<QGraphicsPathItem*>(item);
            QPainterPath path = path_item->path();
            path.moveTo(x, y);
            path_item->setPath(path);
        }
    }
    
    void shawon_graphics_path_item_line_to(void* item, double x, double y) {
        if (item) {
            QGraphicsPathItem* path_item = static_cast<QGraphicsPathItem*>(item);
            QPainterPath path = path_item->path();
            path.lineTo(x, y);
            path_item->setPath(path);
        }
    }
    
    void shawon_graphics_path_item_curve_to(void* item, double cx1, double cy1, double cx2, double cy2, double x, double y) {
        if (item) {
            QGraphicsPathItem* path_item = static_cast<QGraphicsPathItem*>(item);
            QPainterPath path = path_item->path();
            path.cubicTo(cx1, cy1, cx2, cy2, x, y);
            path_item->setPath(path);
        }
    }
    
    void shawon_graphics_path_item_close_path(void* item) {
        if (item) {
            QGraphicsPathItem* path_item = static_cast<QGraphicsPathItem*>(item);
            QPainterPath path = path_item->path();
            path.closeSubpath();
            path_item->setPath(path);
        }
    }
}
