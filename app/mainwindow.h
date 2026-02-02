#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include <memory>
#include "fstree.h"
#include "coverhistory.h"
#include "welcome.h"

namespace XGApp {
	class MainWindow {
    private:
		std::unique_ptr<QMainWindow> mainwindow = std::make_unique<QMainWindow>();
        std::unique_ptr<XGApp::FSTree> fstreeDock =
                    std::make_unique<XGApp::FSTree>();
        std::unique_ptr<XGApp::CoverHistory> coverhistory =
            std::make_unique<XGApp::CoverHistory>();
        XGApp::Welcome* welcome = new XGApp::Welcome;
    public:
		explicit MainWindow();
        explicit MainWindow(MainWindow &) = delete;

        ~MainWindow();

        void show();
	};
}

#endif
