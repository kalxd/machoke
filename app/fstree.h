#ifndef XGAPP_FSTREE
#define XGAPP_FSTREE

#include <QDockWidget>
#include <QTreeView>
#include <QFileSystemModel>
#include <functional>

namespace XGApp {
	class FSTree: public QDockWidget {
    private:
		QTreeView *tree;
        QFileSystemModel *fs;
    public:
		explicit FSTree(QWidget *parent = nullptr);

        void connectPickFile(std::function<void(const QString)> f);
	};
}

#endif
