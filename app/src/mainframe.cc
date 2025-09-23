#include "mainframe.h"
#include <QLabel>
#include <QLayout>
#include <QStackedLayout>

namespace XGApp {
	WelcomeFrame::WelcomeFrame() {
        auto layout = new QVBoxLayout;

        auto label = new QLabel("请选择音频文件，或者将音频文件拖入此处。");
        layout->addWidget(label, 0, Qt::AlignCenter);

        this->widget->setLayout(layout);
    }

    WelcomeFrame::~WelcomeFrame() {
		delete this->widget;
    }
}

namespace XGApp {
	MainFrame::MainFrame() {
		auto stack = new QStackedLayout;

        auto welcome = new XGApp::WelcomeFrame;
        stack->addWidget(welcome->widget);
        this->widget->setLayout(stack);
	}

    MainFrame::~MainFrame() {
        delete this->widget;
    }
}
