#include <QApplication>
#include "mainwindow.h"

int main(int argc, char* argv[]) {
	QApplication app(argc, argv);
	app.setApplicationName("machoke");

    XGApp::MainWindow mainwindow;
    mainwindow.show();

    return app.exec();
}
