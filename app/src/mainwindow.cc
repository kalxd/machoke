#include "mainwindow.h"
#include <QMainWindow>
#include <QToolBar>
#include "lib.rs.h"
#include "mainframe.h"
#include <iostream>
#include <QFileDialog>

namespace XGApp {
	MainWindow::MainWindow() {
        this->setup();
        auto mainWidget = new XGApp::MainFrame;
        this->setCentralWidget(mainWidget);
        this->resize(600, 400);
	}

    MainWindow::~MainWindow() {
		std::cout << this->children().size() << std::endl;
		std::cout << "finish mainwidnow" << std::endl;
    }

    void MainWindow::setup() {
		auto toolbar = this->addToolBar("default");
        toolbar->setFloatable(false);
        toolbar->setMovable(false);

        auto openAction = new QAction("打开文件");
        toolbar->addAction(openAction);
		connect(openAction, &QAction::triggered, this, &MainWindow::pickMedia);
    }

    void MainWindow::pickMedia() {
		QFileDialog dialog(this);
		dialog.exec();
    }
}
