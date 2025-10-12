#include "editor.h"
#include <QBoxLayout>
#include <QGroupBox>
#include <QFormLayout>

namespace XGApp {
	Editor::Editor(QWidget *parent) : QWidget(parent) { this->setup(); }

    QWidget* Editor::setupBasicForm() {
		auto widget = new QGroupBox("基本信息");
        widget->setAlignment(Qt::AlignLeft);

        auto layout = new QFormLayout;

        this->titleLine = new XGApp::Input;
        layout->addRow("标题", this->titleLine);

        widget->setLayout(layout);
		return widget;
    }

    void Editor::setup() {
		auto mainLayout = new QVBoxLayout;

        mainLayout->addWidget(this->setupBasicForm());

        this->setLayout(mainLayout);
    }
}
