#include "mainwidget.h"
#include "widget/util.h"
#include "lib.rs.h"
#include <QLabel>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFormLayout>
#include <QGroupBox>
#include <QDialogButtonBox>

namespace XGApp {
	MainWidget::MainWidget(QWidget *parent) : QStackedWidget(parent) {
        this->welcome = new Welcome;

        this->addWidget(this->welcome);
    }

    void MainWidget::openEditor(::rust::Box<XGLib::Media> &media) {
		if (!this->editor) {
			this->editor = new Editor;
        }
        (*this->editor)->setValue(media);
        this->addWidget(*this->editor);
        this->setCurrentWidget(*this->editor);
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

        auto mainEditorLayout = new QGroupBox("主信息");
        mainLayout->addWidget(mainEditorLayout);

        auto editorFormLayout = new QFormLayout;
        mainEditorLayout->setLayout(editorFormLayout);

        this->title = new QLineEdit;
        editorFormLayout->addRow("名称", this->title);

        this->authorEdits = new XGWidget::MultiEdit;
        editorFormLayout->addRow("作者", this->authorEdits);

        this->album = new QLineEdit;
        editorFormLayout->addRow("专辑", this->album);

        auto btns = new QDialogButtonBox(QDialogButtonBox::Close |
                                             QDialogButtonBox::Save,
                                         Qt::Orientation::Horizontal);
        mainLayout->addWidget(btns, 0, Qt::AlignBottom);

        this->setLayout(mainLayout);
    }

    void MainWidget::Editor::setValue(::rust::Box<XGLib::Media> &media) {
		auto title = media->title();
        this->title->setText(XGWidget::Rust::toString(std::move(title)));

        auto album = media->album();
        this->album->setText(XGWidget::Rust::toString(std::move(album)));

        auto artists = media->artists();
    }
}
