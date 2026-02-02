#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include <memory>
#include "fstree.h"
#include "coverhistory.h"
#include "welcome.h"

namespace XGApp {
	class MainWindow: public QMainWindow {
    private:
        std::unique_ptr<XGApp::FSTree> fstreeDock =
                    std::make_unique<XGApp::FSTree>();
        std::unique_ptr<XGApp::CoverHistory> coverhistory =
            std::make_unique<XGApp::CoverHistory>();
        XGApp::Welcome* welcome = new XGApp::Welcome;
    public:
		explicit MainWindow();
	};
}

#endif
