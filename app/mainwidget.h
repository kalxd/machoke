#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include <QStackedWidget>

namespace XGApp {
    class MainWidget : public QStackedWidget {
    private:
		class Welcome;
		Welcome* welcome;
    public:
        explicit MainWidget(QWidget* parent = nullptr);
    };

    class MainWidget::Welcome : public QWidget {
    public:
		explicit Welcome(QWidget* parent = nullptr);
    };
}

#endif
