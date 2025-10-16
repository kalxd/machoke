#include "cover.h"

namespace XGWidget {
	Cover::Cover(QWidget *parent) : QGroupBox(parent) {
        this->setup();
        this->setTitle("封面");
	}

    void Cover::setup() {
        this->imageLabel = new QLabel(this);
    }
}
