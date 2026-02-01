#include "fstree.h"

namespace XGApp {
	FSTree::FSTree(QWidget *parent) {
        this->fs = new QFileSystemModel;
        this->fs->setRootPath(QDir::rootPath());
        this->tree = new QTreeView;
        this->tree->setModel(this->fs);
        this->tree->setCurrentIndex(this->fs->index(QDir::homePath()));

        this->dock = new QDockWidget("目录", parent);
        this->dock->setAllowedAreas(Qt::LeftDockWidgetArea);
        this->dock->setFeatures(QDockWidget::NoDockWidgetFeatures);
        this->dock->setWidget(this->tree);
    }
}
