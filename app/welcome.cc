#include "welcome.h"
#include <QVBoxLayout>
#include <QLabel>

namespace XGApp {
	Welcome::Welcome(QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QVBoxLayout;
        this->setLayout(mainLayout);

        auto text = new QLabel("欢迎光临！");
        mainLayout->setAlignment(Qt::AlignCenter);
        mainLayout->addWidget(text);
    }
}
