#ifndef XGAPP_MAINWINDOW
#define XGAPP_MAINWINDOW

#include <QMainWindow>
#include <memory>

namespace XGApp {
	class MainWindow {
    private:
        std::unique_ptr<QMainWindow> mainwindow = std::make_unique<QMainWindow>();
    public:
		explicit MainWindow();
        explicit MainWindow(MainWindow &) = delete;

        ~MainWindow();

        void show();
	};
}

#endif
