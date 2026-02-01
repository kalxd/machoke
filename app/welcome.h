#ifndef XGAPP_WELCOME
#define XGAPP_WELCOME

#include <QWidget>

namespace XGApp {
	class Welcome {
    public:
		QWidget* widget;
		explicit Welcome(QWidget* parent = nullptr);
	};
}

#endif
