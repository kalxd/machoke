#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include "fstree.h"
#include "coverhistory.h"
#include "welcome.h"

namespace XGApp {
	class MainWindow: public QMainWindow {
    private:
        XGApp::FSTree* fstreeDock = new XGApp::FSTree;
        XGApp::CoverHistory* coverhistory = new XGApp::CoverHistory();
        XGApp::Welcome* welcome = new XGApp::Welcome;
    public:
		explicit MainWindow();
	};
}

#endif
