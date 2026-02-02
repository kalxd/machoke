#include "mainwindow.h"
#include <iostream>

namespace XGApp {
	MainWindow::MainWindow() {
        this->mainwindow->resize(1200, 800);

        this->mainwindow->addDockWidget(Qt::LeftDockWidgetArea,
                                        this->fstreeDock->dock);
        this->mainwindow->addDockWidget(Qt::RightDockWidgetArea,
                                        this->coverhistory->dock);

        this->mainwindow->setCentralWidget(this->welcome);
	}

    MainWindow::~MainWindow() {}

    void MainWindow::show() {
		this->mainwindow->show();
    }
}
