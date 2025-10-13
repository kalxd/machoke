#include "input.h"

namespace XGApp {
	Input::Input(QWidget *parent) : QLineEdit(parent) { this->setup(); }

    bool Input::isWordInModel(const QString &word) const {
        auto index = this->model->index(0);
        auto indexs = this->model->match(index, 1, word);
        return !indexs.empty();
    }

    void Input::appendToModel(const QString &word) {
		if (this->isWordInModel(word)) {
			return ;
		}
		auto total = this->model->rowCount();
    }

    void Input::setup() {
        this->setCompleter(this->completer);
    }
}
