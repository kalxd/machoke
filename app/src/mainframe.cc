#include "mainframe.h"
#include <QLabel>
#include <QLayout>
#include <QStackedLayout>

namespace XGApp {
	WelcomeFrame::WelcomeFrame(QWidget* parent) : QWidget(parent) {
        auto layout = new QVBoxLayout;

        auto label = new QLabel("请选择音频文件，或者将音频文件拖入此处。");
        layout->addWidget(label, 0, Qt::AlignCenter);

        this->setLayout(layout);
    }
}

namespace XGApp {
	MainFrame::MainFrame(QWidget* parent) : QWidget(parent) {
		auto stack = new QStackedLayout;

        auto welcome = new XGApp::WelcomeFrame;
        stack->addWidget(welcome);
        this->setLayout(stack);
	}
}
