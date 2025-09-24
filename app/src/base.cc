#include "base.h"
#include <iostream>

namespace XGApp {
	BaseWidget::~BaseWidget() {
        delete this->widget;
		std::cout << "finish it";
	}
}
