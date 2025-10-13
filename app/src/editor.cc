#include "editor.h"
#include <QBoxLayout>
#include <QGroupBox>
#include <QFormLayout>
#include <QDialogButtonBox>

namespace XGApp {
	Editor::Editor(QWidget *parent) : QWidget(parent) { this->setup(); }

    QWidget* Editor::setupBasicForm() {
		auto widget = new QGroupBox("基本信息");
        widget->setAlignment(Qt::AlignLeft);

        auto layout = new QFormLayout;

        this->titleLine = new XGApp::Input;
        layout->addRow("标题", this->titleLine);

        this->albumLine = new XGApp::Input;
        layout->addRow("专辑", this->albumLine);

        widget->setLayout(layout);
		return widget;
    }

    void Editor::setup() {
		auto mainLayout = new QVBoxLayout;
        auto editorLayout = new QVBoxLayout;

        editorLayout->addWidget(this->setupBasicForm());
        mainLayout->addLayout(editorLayout, 1);

        auto buttonLayout = new QDialogButtonBox(QDialogButtonBox::Cancel | QDialogButtonBox::Ok);
        mainLayout->addWidget(buttonLayout);

        this->setLayout(mainLayout);
    }
}
