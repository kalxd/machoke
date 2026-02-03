#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include <QStackedWidget>

namespace XGApp {
    class MainWidget : public QStackedWidget {
    private:
		class Welcome;
		class Editor;
		Welcome* welcome;
    public:
        explicit MainWidget(QWidget* parent = nullptr);
    };

    class MainWidget::Welcome : public QWidget {
    public:
		explicit Welcome(QWidget* parent = nullptr);
    };

    class MainWidget::Editor : QWidget {
    public:
		explicit Editor(QWidget* parent = nullptr);
    };
}

#endif
