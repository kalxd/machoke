#include "multiinput.h"
#include <QVBoxLayout>
#include <QHBoxLayout>

namespace XGWidget {
	MultiInputEdit::MultiInputEdit(QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QHBoxLayout;
        mainLayout->addWidget(this->edit, 1);
        mainLayout->addWidget(this->removeBtn);

        this->setLayout(mainLayout);
	}
}

namespace XGWidget {
	MultiInput::MultiInput(QWidget *parent) : QVBoxLayout(parent) {
        this->addWidget(this->fixEdit);
        this->addLayout(this->editLayout);
        this->addWidget(this->addBtn);
	}

	MultiInput::~MultiInput() {
        for (auto edit : this->edits) {
            delete edit;
        }
	}
}
