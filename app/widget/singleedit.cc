#include "singleedit.h"

namespace XGWidget {
	SingleEdit::SingleEdit(QWidget *parent) : QComboBox(parent) {
        this->model = new ComboModel;

        this->setModel(this->model);
        this->setEditable(true);
        this->setInsertPolicy(QComboBox::InsertAtTop);
		this->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
        this->setEditText("");
    }

    QString SingleEdit::getValue() {
		auto value = this->currentText().trimmed();
        if (not value.isEmpty()) {
			this->model->appendWord(value);
		}

        return value;
    }

    void SingleEdit::setValue(const QString &word) {
		this->setEditText(word);

        if (not word.isEmpty()) {
			this->model->appendWord(word);
		}
    }
}
