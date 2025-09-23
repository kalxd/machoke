#include "mainwindow.h"
#include <QMainWindow>
#include <QToolBar>
#include "lib.rs.h"
#include "mainframe.h"

namespace XGApp {
	MainWindow::MainWindow() {
        this->setup();
        auto mainWidget = new XGApp::MainFrame;
        this->setCentralWidget(mainWidget->widget);
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
