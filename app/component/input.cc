#include "input.h"
#include <QDebug>

namespace XGApp {
	Input::Input(QWidget *parent) : QLineEdit(parent) { this->setup(); }

    bool Input::isWordInModel(const QString &word) const {
        auto index = this->model->index(0);
        auto indexs = this->model->match(index, Qt::DisplayRole, word);
        return !indexs.empty();
    }

    void Input::appendToModel(const QString &word) {
		if (this->isWordInModel(word)) {
            return ;
		}

        auto total = this->model->rowCount();
        if (this->model->insertRow(total)) {
			auto index = this->model->index(total);
			this->model->setData(index, word);
        }
    }

    void Input::setup() {
		this->completer->setCompletionMode(QCompleter::UnfilteredPopupCompletion);
		this->setCompleter(this->completer);
    }

    QString Input::text() {
		auto lineText = this->QLineEdit::text().trimmed();
		this->appendToModel(lineText);
		return lineText;
    }
}
