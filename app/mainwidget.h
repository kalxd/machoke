#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include "welcome.h"
#include <QStackedWidget>

namespace XGApp {
	class MainWidget : public QStackedWidget {
    private:
        XGApp::Welcome* welcome;
    public:
        explicit MainWidget(QWidget* parent = nullptr);
	};
}

#endif
