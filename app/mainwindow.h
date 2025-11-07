#ifndef _XG_MAINWINDOW_
#define _XG_MAINWINDOW_

#include <QMainWindow>
#include <optional>
#include "lib.rs.h"
#include "mainframe.h"

namespace XGApp {
	class MainWindow : public QMainWindow {
		Q_OBJECT
    public:
		MainWindow();
        ~MainWindow();

    private:
		void setup();
        void pickMedia();

        XGApp::MainFrame* mainFrame;

        std::optional<rust::Box<XGLib::Media>> media = std::nullopt;
	};
}

#endif
