#include <QMainWindow>
#include <QLayout>
#include <QPushButton>

namespace MainWindow {
	void setupUi(QMainWindow &w) {
		auto button = new QPushButton("click");
		w.setCentralWidget(button);
	}
}
