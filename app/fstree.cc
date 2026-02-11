#include "fstree.h"
#include <QHeaderView>
#include <QDebug>

namespace XGApp {
	FSTree::FSTree(QWidget *parent) : QDockWidget(parent) {
        this->fs = new QFileSystemModel;
        this->fs->setRootPath(QDir::rootPath());
        this->fs->setReadOnly(true);
        this->fs->setNameFilters({"*.mp3"});
        this->fs->setNameFilterDisables(false);

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

    void FSTree::connectPickFile(std::function<void(const QString)> f) {
      connect(this->tree, &QTreeView::doubleClicked, this,
              [this, f](const QModelIndex &index) {
				  if (not index.isValid()) {
                      return;
                  }

                  if (this->fs->isDir(index)) {
                      return ;
                  }

                  f(this->fs->filePath(index));
			  });
    }
}
