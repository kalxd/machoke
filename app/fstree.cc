#include "fstree.h"
#include <QHeaderView>

namespace XGApp {
	FSTree::FSTree(QWidget *parent) : QDockWidget(parent) {
        this->fs = new QFileSystemModel;
        this->fs->setRootPath(QDir::rootPath());
        this->tree = new QTreeView;
        this->tree->setModel(this->fs);
        this->tree->setCurrentIndex(this->fs->index(QDir::homePath()));
        this->tree->setColumnHidden(1, true); // 文件大小
        this->tree->setColumnHidden(2, true); // 文件类型
        this->tree->setColumnHidden(3, true); // 文件大小
        this->tree->setHeaderHidden(true);
        this->tree->header()->setSectionResizeMode(0, QHeaderView::ResizeToContents);
        this->tree->header()->setStretchLastSection(false);

        this->setAllowedAreas(Qt::LeftDockWidgetArea);
        this->setFeatures(QDockWidget::NoDockWidgetFeatures);
        this->setWidget(this->tree);
        this->setWindowTitle("目录");
    }
}
