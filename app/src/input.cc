#include "input.h"

namespace XGApp {
	Input::Input(QWidget *parent) : QLineEdit(parent) {
		this->setup();
	}

    void Input::setup() {
        this->setCompleter(this->completer);
    }
}
