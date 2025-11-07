#include "mainwindow.h"
#include <QMainWindow>
#include <QToolBar>
#include "lib.rs.h"
#include "mainframe.h"
#include <QFileDialog>
#include <exception>
#include <QErrorMessage>

namespace XGApp {
	MainWindow::MainWindow() : mainFrame(new XGApp::MainFrame) {
        this->setup();
        this->setCentralWidget(this->mainFrame);
        this->resize(800, 600);
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

        try {
			auto media = XGLib::readAudioFile(selectFile.toStdString());
            this->media = std::move(media);
        } catch (const std::exception &e) {
			QErrorMessage dialog(this);
            dialog.showMessage(e.what());
            dialog.exec();
            return ;
        }

        this->mainFrame->showEditor();
    }
}
