#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include <memory>
#include "fstree.h"

namespace XGApp {
	class MainWindow {
    private:
		std::unique_ptr<QMainWindow> mainwindow = std::make_unique<QMainWindow>();
		std::unique_ptr<XGApp::FSTree> fstreeDock = std::make_unique<XGApp::FSTree>();
    public:
		explicit MainWindow();
        explicit MainWindow(MainWindow &) = delete;

        ~MainWindow();

        void show();
	};
}

#endif
