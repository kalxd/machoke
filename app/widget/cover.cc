#include "cover.h"
#include <QVBoxLayout>
#include <QPushButton>
#include <QFileDialog>
#include <QDebug>

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

        this->setLayout(mainLayout);
    }

    void Cover::chooseCover() {
		auto filename = QFileDialog::getOpenFileName(this, "打开图片", QDir::homePath(),
                                                   "图片 (*.png *.jpg *.jpeg)");

		if (filename.isEmpty()) {
			return ;
        }

        auto pixmap = new QPixmap(filename);
        this->coverLabel->setPixmap(*pixmap);
        this->pixmap.emplace(pixmap);
    }
}
