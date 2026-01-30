#include <QApplication>
#include <QWindow>

int main(int argc, char* argv[]) {
	QApplication app(argc, argv);

    QWindow window;
    // window.resize(800, 600);
    window.show();

    return app.exec();
}
