#include <QApplication>
#include <QMainWindow>
#include "mainwindow.h"

int main(int argc, char *argv[]) {
	QApplication app(argc, argv);

    XGApp::MainWindow w;
    w.show();

	// QMainWindow w;

	// MainWindow::setupUi(w);

	// w.resize(600, 400);
	// w.show();

	app.exec();
}
