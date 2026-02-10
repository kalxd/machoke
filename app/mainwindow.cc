#include "mainwindow.h"
#include <QMessageBox>
#include <QStatusBar>

namespace XGApp {
	MainWindow::MainWindow() {
        this->resize(1200, 800);

        this->addDockWidget(Qt::LeftDockWidgetArea, this->fstreeDock);
        this->addDockWidget(Qt::RightDockWidgetArea, this->coverhistory);

        this->setCentralWidget(this->mainWidget);
        connect(this->mainWidget, &MainWidget::failed, this,
                &MainWindow::showFailMsg);
        connect(this->mainWidget, &MainWidget::saved, this, &MainWindow::showOkMsg);
		this->showReadyMsg();

        this->fstreeDock->connectPickFile(std::bind(&MainWindow::openAudio,
                                                    this,
                                                    std::placeholders::_1));
    }

    void MainWindow::openAudio(const QString path) {
        auto media = XGLib::readAudioFile(path.toStdString());
        this->mainWidget->openEditor(std::move(media));
        this->showReadyMsg();
        this->media = std::move(media);
    }

    void MainWindow::showReadyMsg() {
		this->statusBar()->showMessage("准备就绪！");
    }

    void MainWindow::showFailMsg(const QString errorMsg) {
		auto msg = QString("保存失败： %1").arg(errorMsg);
		this->statusBar()->showMessage(msg);
    }

    void MainWindow::showOkMsg() {
		this->statusBar()->showMessage("保存成功！");
    }
}
