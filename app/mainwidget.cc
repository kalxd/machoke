#include "mainwidget.h"

namespace XGApp {
	MainWidget::MainWidget(QWidget *parent) : QStackedWidget(parent) {
        this->welcome = new XGApp::Welcome;

        this->addWidget(this->welcome);
	}
}
