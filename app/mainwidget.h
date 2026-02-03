#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include <QStackedWidget>
#include <QLineEdit>
#include <optional>

namespace XGApp {
    class MainWidget : public QStackedWidget {
    private:
		class Welcome;
		class Editor;
        Welcome *welcome;
        std::optional<Editor*> editor = std::nullopt;

    public:
		explicit MainWidget(QWidget *parent = nullptr);

        void openEditor();
    };

    class MainWidget::Welcome : public QWidget {
    public:
		explicit Welcome(QWidget* parent = nullptr);
    };

    class MainWidget::Editor : public QWidget {
    private:
		QLineEdit* title;
    public:
		explicit Editor(QWidget* parent = nullptr);
    };
}

#endif
