#ifndef _XG_MAINWINDOW_
#define _XG_MAINWINDOW_

#include <QMainWindow>

namespace XGApp {
	class MainWindow : public QMainWindow {
		Q_OBJECT
    public:
		MainWindow();
        ~MainWindow();

    private:
		void setup();
		void pickMedia();
	};
}

#endif
