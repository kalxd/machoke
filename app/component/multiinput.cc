#include "multiinput.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QDebug>

namespace XGWidget {
	MultiInputEdit::MultiInputEdit(QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QHBoxLayout;
        mainLayout->addWidget(this->edit, 1);
        mainLayout->addWidget(this->removeBtn);

		connect(this->removeBtn, &QPushButton::clicked, this, [this]() { emit this->remove(); });

        this->setLayout(mainLayout);
    }

    QString MultiInputEdit::text() const { return this->edit->text(); }
}

namespace XGWidget {
	MultiInput::MultiInput(QWidget *parent) : QWidget(parent) {
		auto mainLayout = new QVBoxLayout;
        mainLayout->addWidget(this->fixEdit);
        mainLayout->addLayout(this->editLayout);
        mainLayout->addWidget(this->addBtn);
        this->setLayout(mainLayout);

        connect(this->addBtn, &QPushButton::clicked, this, &MultiInput::addEditLine);
	}

	MultiInput::~MultiInput() {
        for (auto edit : this->edits) {
            delete edit;
        }
    }

    void MultiInput::addEditLine() {
		auto edit = new MultiInputEdit;

        this->edits.append(edit);
        this->editLayout->addWidget(edit);
		connect(edit, &MultiInputEdit::remove, this, [this, edit]() { this->removeEditLine(edit); });
    }

    void MultiInput::removeEditLine(MultiInputEdit *edit) {
		this->editLayout->removeWidget(edit);
    }

    std::vector<std::string> MultiInput::texts() const {
		std::vector<std::string> out;

        for (const auto &item : this->edits) {
			auto text = item->text().trimmed();

            if (!text.isEmpty()) {
				out.push_back(text.toStdString());
			}
        }

        return out;
    }
}
