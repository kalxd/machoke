#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include <QStackedWidget>
#include <QLineEdit>
#include <optional>
#include "lib.rs.h"
#include "widget/cover.h"
#include "widget/multiedit.h"
#include "widget/singleedit.h"

namespace XGApp {
    class MainWidget : public QStackedWidget {
    private:
		class Welcome;
		class Editor;
        Welcome *welcome;
        std::optional<Editor*> editor = std::nullopt;
    public:
		explicit MainWidget(QWidget *parent = nullptr);

        void openEditor(::rust::Box<XGLib::Media> &&media);
    };

    class MainWidget::Welcome : public QWidget {
    public:
		explicit Welcome(QWidget* parent = nullptr);
    };

    class MainWidget::Editor : public QWidget {
    private:
		std::optional<::rust::Box<XGLib::Media>> media = std::nullopt;

		XGWidget::Cover* cover;
        QLineEdit *title;
        XGWidget::MultiEdit *artistEdits;
        XGWidget::SingleEdit *album;
        XGWidget::MultiEdit *genreEdits;
		void save();
    public:
		explicit Editor(QWidget *parent = nullptr);

        void setValue(::rust::Box<XGLib::Media> &&media);
    };
}

#endif
