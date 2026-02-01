#include "fstree.h"

namespace XGApp {
	FSTree::FSTree(QWidget *parent) {
		this->dock = new QDockWidget("目录", parent);

        this->dock->setAllowedAreas(Qt::LeftDockWidgetArea);
		this->dock->setFeatures(QDockWidget::NoDockWidgetFeatures);
    }
}
