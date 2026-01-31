#include "mainwindow.h"
#include <qnamespace.h>

namespace XGApp {
	MainWindow::MainWindow() {
        this->mainwindow->resize(1200, 800);

		this->mainwindow->addDockWidget(Qt::LeftDockWidgetArea, this->fstreeDock->dock);
	}

    MainWindow::~MainWindow() {}

    void MainWindow::show() {
		this->mainwindow->show();
    }
}
