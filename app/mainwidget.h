#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include <QStackedWidget>
#include <QLineEdit>
#include <optional>
#include "lib.rs.h"
#include "widget/cover.h"

namespace XGApp {
    class MainWidget : public QStackedWidget {
    private:
		class Welcome;
		class Editor;
        Welcome *welcome;
        std::optional<Editor*> editor = std::nullopt;

    public:
		explicit MainWidget(QWidget *parent = nullptr);

        void openEditor(const std::optional<XGLib::Media*> &media);
    };

    class MainWidget::Welcome : public QWidget {
    public:
		explicit Welcome(QWidget* parent = nullptr);
    };

    class MainWidget::Editor : public QWidget {
    private:
		XGWidget::Cover* cover;
		QLineEdit* title;
    public:
		explicit Editor(QWidget *parent = nullptr);

        void setValue(const std::optional<XGLib::Media*> &media);
    };
}

#endif
