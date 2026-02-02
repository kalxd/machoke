#ifndef XGAPP_FSTREE
#define XGAPP_FSTREE

#include <QDockWidget>
#include <QTreeView>
#include <QFileSystemModel>

namespace XGApp {
	class FSTree: public QDockWidget {
    private:
		QTreeView *tree;
        QFileSystemModel *fs;
    public:
		explicit FSTree(QWidget* parent = nullptr);
	};
}

#endif
