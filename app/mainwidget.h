#ifndef XGAPP_MAINWIDGET
#define XGAPP_MAINWIDGET

#include <QStackedWidget>
#include <QLineEdit>
#include <optional>
#include "lib.rs.h"
#include "rust/util.h"
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
        void updateCover(const XGRust::CoverInfo info);
    public:
		explicit MainWidget(QWidget *parent = nullptr);

        void openEditor(::rust::Box<XGLib::Media> &&media);
        void closeEditor();
        void setCover(const XGRust::CoverInfo &&info);
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
		void applyMediaInfo();
	signals:
        void closed();
        void saved();
        void failed(const QString);
        void updateCover(const XGRust::CoverInfo info);
    public:
		explicit Editor(QWidget *parent = nullptr);

        void setValue(::rust::Box<XGLib::Media> &&media);
        void setCover(const XGRust::CoverInfo &&info);
    };
}

#endif
