#include "welcome.h"
#include <QVBoxLayout>
#include <QLabel>

namespace XGApp {
	Welcome::Welcome(QWidget *parent) {
        this->widget = new QWidget(parent);

        auto mainLayout = new QVBoxLayout;
        this->widget->setLayout(mainLayout);

        auto text = new QLabel("欢迎光临！");
        mainLayout->setAlignment(Qt::AlignCenter);
        mainLayout->addWidget(text);
	}
}
