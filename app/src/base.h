#ifndef _XG_BASE_
#define _XG_BASE_

#include <QWidget>

namespace XGApp {
	class BaseWidget {
    public:
		QWidget *widget = new QWidget;

        ~BaseWidget();
	};
}

#endif
