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

        layout->addRow("标题", this->titleLine);
        layout->addRow("作者", this->authorLines);

        this->albumLine = new XGApp::Input;
        layout->addRow("专辑", this->albumLine);
        layout->addRow("流派", this->genreLines);

        widget->setLayout(layout);
		return widget;
    }

    void Editor::setup() {
		auto mainLayout = new QVBoxLayout;

        auto coverLayout = new QHBoxLayout;

        this->cover = new XGWidget::Cover;
        coverLayout->addWidget(this->cover, 3);

        this->coverHistory = new XGWidget::CoverHistory;
        coverLayout->addWidget(this->coverHistory, 2);

        mainLayout->addLayout(coverLayout);

        auto editorLayout = new QVBoxLayout;

        editorLayout->addWidget(this->setupBasicForm());
        mainLayout->addLayout(editorLayout, 1);

        auto buttonLayout = new QDialogButtonBox(QDialogButtonBox::Cancel | QDialogButtonBox::Ok);
        mainLayout->addWidget(buttonLayout);

        this->setLayout(mainLayout);
    }
}
