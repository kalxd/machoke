#include "mainwindow.h"

namespace XGApp {
	MainWindow::MainWindow() {
		this->mainwindow->resize(1200, 800);
	}

    MainWindow::~MainWindow() {}

    void MainWindow::show() {
		this->mainwindow->show();
    }
}
