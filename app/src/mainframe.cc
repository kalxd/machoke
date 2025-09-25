#include "mainframe.h"
#include "editor.h"
#include <QLabel>
#include <QLayout>
#include <qstackedwidget.h>

namespace XGApp {
	WelcomeFrame::WelcomeFrame(QWidget* parent) : QWidget(parent) {
        auto layout = new QVBoxLayout;

        auto label = new QLabel("请选择音频文件，或者将音频文件拖入此处。");
        layout->addWidget(label, 0, Qt::AlignCenter);

        this->setLayout(layout);
    }
}

namespace XGApp {
	MainFrame::MainFrame(QWidget* parent) : QStackedWidget(parent) {
        auto welcome = new XGApp::WelcomeFrame;
        this->addWidget(welcome);

        auto editor = new XGApp::Editor;
        this->addWidget(editor);
    }

    void MainFrame::showWelcome() { this->setCurrentIndex(0); }

    void MainFrame::showEditor() {
		this->setCurrentIndex(1);
    }
}
