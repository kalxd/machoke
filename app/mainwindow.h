#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include <optional>
#include "fstree.h"
#include "coverhistory.h"
#include "mainwidget.h"
#include "lib.rs.h"

namespace XGApp {
	class MainWindow: public QMainWindow {
    private:
        XGApp::FSTree* fstreeDock = new XGApp::FSTree;
        XGApp::CoverHistory *coverhistory = new XGApp::CoverHistory();
        XGApp::MainWidget *mainWidget = new XGApp::MainWidget;

        std::optional<::rust::Box<XGLib::Media>> media = std::nullopt;

        void openAudio(const QString path);

    public:
		explicit MainWindow();
	};
}

#endif
