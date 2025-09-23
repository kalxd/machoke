#include "mainwindow.h"
#include <QMainWindow>
#include <QToolBar>
#include "lib.rs.h"

namespace XGApp {
	MainWindow::MainWindow() {
		this->setup();
        this->resize(600, 400);
	}

    MainWindow::~MainWindow() {}

    void MainWindow::setup() {
		auto toolbar = this->addToolBar("default");
        toolbar->setFloatable(false);
        toolbar->setMovable(false);

        auto openAction = new QAction("打开文件");
        toolbar->addAction(openAction);
        connect(openAction, &QAction::triggered, this, [](){ XGLib::sayHello(); });
    }
}
