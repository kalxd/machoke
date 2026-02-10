#include "cover.h"
#include "lib.rs.h"
#include "../rust/util.h"
#include <QVBoxLayout>
#include <QPushButton>
#include <QFileDialog>

namespace XGWidget {
	Cover::Cover(QWidget *parent) : QGroupBox(parent) {
        this->setTitle("封面");

        auto mainLayout = new QVBoxLayout;

        this->coverLabel = new QLabel;
        this->coverLabel->setFixedSize(256, 256);
        mainLayout->addWidget(this->coverLabel, 1, Qt::AlignCenter);

        auto changeBtn = new QPushButton("更换封面");
        mainLayout->addWidget(changeBtn);
        connect(changeBtn, &QPushButton::clicked, this, &Cover::chooseCover);

        auto removeBtn = new QPushButton("删除封面");
        mainLayout->addWidget(removeBtn);
        connect(removeBtn, &QPushButton::clicked, this, &Cover::removeCover);

        this->setLayout(mainLayout);
    }

    void Cover::loadPixmap(const QPixmap *pixmap) {
      this->coverLabel->setPixmap(
								  pixmap->scaled(256, 256, Qt::KeepAspectRatio));
	  this->pixmap.emplace(pixmap);
    }

    void Cover::chooseCover() {
		auto filename = QFileDialog::getOpenFileName(this, "打开图片", QDir::homePath(),
                                                   "图片 (*.png *.jpg *.jpeg)");

		if (filename.isEmpty()) {
			return ;
        }

        auto file = QFileInfo(filename);
        auto pixmap = new QPixmap(filename);
        this->loadPixmap(pixmap);
        this->mime = XGRust::mimeFromString(file.suffix());
    }

    void Cover::removeCover() {
		this->coverLabel->clear();
        this->pixmap.reset();
        this->mime = XGLib::CoverMime::None;
    }

    void Cover::setValue(const ::rust::Box<XGLib::CoverTuple> &&cover) {
		if (cover->mime == XGLib::CoverMime::None) {
            this->pixmap.reset();
            this->coverLabel->clear();
            this->mime = XGLib::CoverMime::None;
            return ;
        }

        auto pixmap = new QPixmap;
        auto picData = XGRust::toByteArray(std::move(cover->data));
        if (pixmap->loadFromData(picData, XGRust::toMimeString(cover->mime))) {
			this->loadPixmap(pixmap);
			this->mime = cover->mime;
        } else {
			delete pixmap;
        }
    }
}
