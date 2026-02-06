#include "mainwidget.h"
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

    void MainWidget::openEditor(const std::optional<XGLib::Media *> &media) {
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

        auto btns = new QDialogButtonBox(QDialogButtonBox::Close |
                                             QDialogButtonBox::Save,
                                         Qt::Orientation::Horizontal);
        mainLayout->addWidget(btns, 0, Qt::AlignBottom);

        this->setLayout(mainLayout);
    }

    void MainWidget::Editor::setValue(
									  const std::optional<XGLib::Media *> &media) {
		if (media) {
            auto title = XGLib::readMediaTitle(*media);
            this->title->setText(QString::fromStdString((std::string)title));
        } else {
            this->title->setText("");
        }
    }
}
