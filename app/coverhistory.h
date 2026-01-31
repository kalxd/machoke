#ifndef XGAPP_COVERHISTORY
#define XGAPP_COVERHISTORY

#include <QDockWidget>

namespace XGApp {
	class CoverHistory {
    public:
		QDockWidget *dock;

        explicit CoverHistory(QWidget* parent = nullptr);
	};
}

#endif
