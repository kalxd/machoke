#include "mainwindow.h"
#include <QMainWindow>
#include <QToolBar>
#include "lib.rs.h"
#include "mainframe.h"
#include <QFileDialog>

namespace XGApp {
	MainWindow::MainWindow() {
        this->setup();
        auto mainWidget = new XGApp::MainFrame;
        this->setCentralWidget(mainWidget);
        this->resize(600, 400);
	}

    MainWindow::~MainWindow() {
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
		auto selectFile = QFileDialog::getOpenFileName(this, "打开音频",
                                                       QString(), "音频 (*.mp3)");

		if (selectFile == nullptr) {
			return ;
        }

        auto media = XGLib::readAudioFile(selectFile.toStdString());
        this->media = std::move(media);
    }
}
