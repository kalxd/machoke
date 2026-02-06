#include "cover.h"
#include <QVBoxLayout>

namespace XGWidget {
	Cover::Cover(QWidget *parent) : QGroupBox(parent) {
        this->setTitle("封面");

        auto mainLayout = new QVBoxLayout;

        this->coverLabel = new QLabel;
        this->coverLabel->setFixedSize(256, 256);
        mainLayout->addWidget(this->coverLabel);

        this->setLayout(mainLayout);
	}
}
