#include "mainwindow.h"
#include <QMessageBox>
#include <QStatusBar>
#include "lib.rs.h"
#include "rust/util.h"

namespace XGApp {
	MainWindow::MainWindow() {
        this->resize(1200, 800);

        this->addDockWidget(Qt::LeftDockWidgetArea, this->fstreeDock);
        this->addDockWidget(Qt::RightDockWidgetArea, this->coverhistory);

        this->setCentralWidget(this->mainWidget);
        connect(this->mainWidget, &MainWidget::failed, this,
                &MainWindow::showFailMsg);
        connect(this->mainWidget, &MainWidget::saved, this,
                &MainWindow::showOkMsg);
        connect(this->mainWidget, &MainWidget::updateCover, this,
                &MainWindow::updateCoverIcon);
        this->coverhistory->connectChoose([this](const auto info) {
			this->mainWidget->setCover(std::move(info));
        });

        this->showReadyMsg();
        this->fstreeDock->connectPickFile(std::bind(&MainWindow::openAudio,
                                                    this,
                                                    std::placeholders::_1));
    }

    void MainWindow::openAudio(const QString path) {
		auto media = XGLib::readAudioFile(path.toStdString());
        {
            auto cover = media->front_cover();
            if (cover->mime != XGLib::CoverMime::None) {
				auto bs = XGRust::toByteArray(cover->data);
                QPixmap pixmap;
                pixmap.loadFromData(bs, XGRust::toMimeString(cover->mime));

                XGRust::CoverInfo info{
					.pixmap = pixmap,
					.mime = cover->mime,
					.path = path
                };

                this->coverhistory->appendCover(std::move(info));
            }
        }

        this->mainWidget->openEditor(std::move(media));
        this->showReadyMsg();
        this->media = std::move(media);
    }

    void MainWindow::updateCoverIcon(const XGRust::CoverInfo info) {
		this->coverhistory->appendCover(std::move(info));
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
