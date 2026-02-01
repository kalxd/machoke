#include "fstree.h"
#include <QHeaderView>

namespace XGApp {
	FSTree::FSTree(QWidget *parent) {
        this->fs = new QFileSystemModel;
        this->fs->setRootPath(QDir::rootPath());
        this->tree = new QTreeView;
        this->tree->setModel(this->fs);
        this->tree->setCurrentIndex(this->fs->index(QDir::homePath()));
        this->tree->setColumnHidden(1, true); // 文件大小
        this->tree->setColumnHidden(2, true); // 文件类型
        this->tree->setColumnHidden(3, true); // 文件大小
        this->tree->header()->setSectionResizeMode(0, QHeaderView::ResizeToContents);
        this->tree->header()->setStretchLastSection(false);

        this->dock = new QDockWidget("目录", parent);
        this->dock->setAllowedAreas(Qt::LeftDockWidgetArea);
        this->dock->setFeatures(QDockWidget::NoDockWidgetFeatures);
        this->dock->setWidget(this->tree);
    }
}
