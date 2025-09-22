#ifndef _XG_MAINWINDOW_
#define _XG_MAINWINDOW_

#include <QMainWindow>
#include <qmainwindow.h>

namespace XGApp {
	class MainWindow {
    public:
		MainWindow();
        ~MainWindow();

        void show();

    private:
        QMainWindow w;
	};
}

#endif
