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
		Q_OBJECT
    private:
		class Welcome;
		class Editor;
        Welcome *welcome;
        std::optional<Editor *> editor = std::nullopt;
	signals:
        void saved();
        void failed(const QString);
    public:
		explicit MainWidget(QWidget *parent = nullptr);

        void openEditor(::rust::Box<XGLib::Media> &&media);
        void closeEditor();
    };

    class MainWidget::Welcome : public QWidget {
    public:
		explicit Welcome(QWidget* parent = nullptr);
    };

    class MainWidget::Editor : public QWidget {
		Q_OBJECT
    private:
		std::optional<::rust::Box<XGLib::Media>> media = std::nullopt;

		XGWidget::Cover* cover;
        QLineEdit *title;
        XGWidget::MultiEdit *artistEdits;
        XGWidget::SingleEdit *album;
        XGWidget::MultiEdit *genreEdits;

        void save();
	signals:
        void closed();
        void saved();
        void failed(const QString);
    public:
		explicit Editor(QWidget *parent = nullptr);

        void setValue(::rust::Box<XGLib::Media> &&media);
    };
}

#endif
