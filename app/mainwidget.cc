#include "mainwidget.h"
#include <QLabel>
#include <QVBoxLayout>
#include <QDialogButtonBox>

namespace XGApp {
	MainWidget::MainWidget(QWidget *parent) : QStackedWidget(parent) {
        this->welcome = new Welcome;

        this->addWidget(this->welcome);
    }

    void MainWidget::openEditor() {
		auto editor = new Editor;
        this->addWidget(editor);
        this->setCurrentWidget(editor);
    }

    MainWidget::Welcome::Welcome(QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QVBoxLayout;

        auto text = new QLabel("选择文件开始编辑。");
        mainLayout->setAlignment(Qt::AlignCenter);
        mainLayout->addWidget(text);

        this->setLayout(mainLayout);
    }

    MainWidget::Editor::Editor(QWidget *parent) : QWidget(parent) {
		auto mainLayout = new QVBoxLayout;

        this->cover = new XGWidget::Cover;
        mainLayout->addWidget(this->cover);

        this->title = new QLineEdit;
        mainLayout->addWidget(this->title);

        auto btns = new QDialogButtonBox(QDialogButtonBox::Close |
                                             QDialogButtonBox::Save,
                                         Qt::Orientation::Horizontal);
        mainLayout->addWidget(btns, 0, Qt::AlignBottom);

        this->setLayout(mainLayout);
    }
}
