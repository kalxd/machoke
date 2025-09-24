#ifndef _XG_MAINFRAME_
#define _XG_MAINFRAME_

#include <QWidget>
#include <qtmetamacros.h>

namespace XGApp {

	class WelcomeFrame : public QWidget {
		Q_OBJECT
    public:
        explicit WelcomeFrame(QWidget *parent = nullptr);
    };

    class MainFrame : public QWidget {
		Q_OBJECT
    public:
        explicit MainFrame(QWidget* parent = nullptr);
	};
}

#endif
