#include "multiedit.h"

namespace XGWidget {
	MultiEdit::MultiEdit(QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QVBoxLayout;
        mainLayout->setContentsMargins(0, 0, 0, 0);

        this->model = new QStringListModel;

        this->firstCombo = this->createCombox();
        mainLayout->addWidget(this->firstCombo);

        this->expandLayout = new QVBoxLayout;
        mainLayout->addLayout(this->expandLayout);

        this->addBtn = new QPushButton("添加新列");
        connect(this->addBtn, &QPushButton::clicked, this, &MultiEdit::addBlankLine);
        mainLayout->addWidget(this->addBtn);

        this->setLayout(mainLayout);
    }

    QComboBox *MultiEdit::createCombox(QWidget *parent) const {
		auto combo = new QComboBox(parent);
        combo->setModel(this->model);
        combo->setEditable(true);
        combo->setInsertPolicy(QComboBox::InsertAtTop);
        combo->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
        combo->setEditText("");
        return combo;
    }

    void MultiEdit::addBlankLine() {
		auto combo = this->createCombox();
        this->expandBoxs << combo;
        this->expandLayout->addWidget(combo);
    }

    void MultiEdit::setValues(const QStringList &xs) {
		auto raws = this->model->stringList();
        raws << xs;
        raws.removeDuplicates();
        this->model->setStringList(raws);

        for (const auto &item : this->expandBoxs) {
			this->expandLayout->removeWidget(item);
        }

        this->expandBoxs.clear();
    }
}
