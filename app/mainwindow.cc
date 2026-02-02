#include "mainwindow.h"
#include "lib.rs.h"
#include <exception>
#include <QMessageBox>

namespace XGApp {
	MainWindow::MainWindow() {
        this->resize(1200, 800);

        this->addDockWidget(Qt::LeftDockWidgetArea, this->fstreeDock);
        this->addDockWidget(Qt::RightDockWidgetArea, this->coverhistory);

        this->setCentralWidget(this->welcome);

        this->fstreeDock->connectPickFile(std::bind(&MainWindow::openAudio,
                                                    this,
                                                    std::placeholders::_1));
    }

    void MainWindow::openAudio(const QString path) {
		try {
			auto media = XGLib::readAudioFile(path.toStdString());
        } catch (const std::exception &e) {
            QMessageBox msg(this);
            msg.setIcon(QMessageBox::Critical);
            msg.setText("无法打开音频！");
			msg.setDetailedText(e.what());
            msg.exec();
        }
    }
}
