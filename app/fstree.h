#ifndef XGAPP_FSTREE
#define XGAPP_FSTREE

#include <QDockWidget>

namespace XGApp {
	class FSTree {
    public:
		QDockWidget* dock;
		explicit FSTree(QWidget* parent = nullptr);
	};
}

#endif
