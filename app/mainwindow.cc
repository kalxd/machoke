#include "mainwindow.h"
#include <QMessageBox>

namespace XGApp {
	MainWindow::MainWindow() {
        this->resize(1200, 800);

        this->addDockWidget(Qt::LeftDockWidgetArea, this->fstreeDock);
        this->addDockWidget(Qt::RightDockWidgetArea, this->coverhistory);

        this->setCentralWidget(this->mainWidget);

        this->fstreeDock->connectPickFile(std::bind(&MainWindow::openAudio,
                                                    this,
                                                    std::placeholders::_1));
    }

    void MainWindow::openAudio(const QString path) {
        auto media = XGLib::readAudioFile(path.toStdString());
        this->mainWidget->openEditor(std::move(media));
        this->media = std::move(media);
    }
}
