#ifndef XGAPP_FSTREE
#define XGAPP_FSTREE

#include <QDockWidget>
#include <QTreeView>
#include <QFileSystemModel>

namespace XGApp {
	class FSTree {
    private:
		QTreeView *tree;
		QFileSystemModel* fs;
    public:
		QDockWidget* dock;
		explicit FSTree(QWidget* parent = nullptr);
	};
}

#endif
