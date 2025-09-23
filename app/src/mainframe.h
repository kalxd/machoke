#ifndef _XG_MAINFRAME_
#define _XG_MAINFRAME_

#include <QWidget>

namespace XGApp {
	class WelcomeFrame {
    public:
		explicit WelcomeFrame();
		~WelcomeFrame();

        QWidget* widget = new QWidget;
    };

    class MainFrame {
    public:
        MainFrame();
        ~MainFrame();

        QWidget* widget = new QWidget;
	};
}

#endif
