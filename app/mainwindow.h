#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include "fstree.h"
#include "coverhistory.h"
#include "mainwidget.h"

namespace XGApp {
	class MainWindow: public QMainWindow {
    private:
        XGApp::FSTree* fstreeDock = new XGApp::FSTree;
        XGApp::CoverHistory *coverhistory = new XGApp::CoverHistory();
        XGApp::MainWidget* mainWidget = new XGApp::MainWidget;

        void openAudio(const QString path);

    public:
		explicit MainWindow();
	};
}

#endif
