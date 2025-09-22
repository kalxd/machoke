#include "mainwindow.h"
#include <QMainWindow>

namespace XGApp {
	MainWindow::MainWindow() {
		this->w.resize(600, 400);
	}

    MainWindow::~MainWindow() { }

    void MainWindow::show() {
		this->w.show();
    }
}
