#include "mainwindow.h"

namespace XGApp {
	MainWindow::MainWindow() {
        this->resize(1200, 800);

        this->addDockWidget(Qt::LeftDockWidgetArea,
                                        this->fstreeDock->dock);
        this->addDockWidget(Qt::RightDockWidgetArea,
                                        this->coverhistory->dock);

        this->setCentralWidget(this->welcome);
	}
}
