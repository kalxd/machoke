#include "mainwidget.h"
#include "widget/multiedit.h"
#include "rust/util.h"
#include "lib.rs.h"
#include <QLabel>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFormLayout>
#include <QGroupBox>
#include <QDialogButtonBox>
#include <QMessageBox>

namespace XGApp {
	MainWidget::MainWidget(QWidget *parent) : QStackedWidget(parent) {
        this->welcome = new Welcome;

        this->addWidget(this->welcome);
    }

    void MainWidget::openEditor(::rust::Box<XGLib::Media> &&media) {
		if (!this->editor) {
            this->editor = new Editor;
            (*this->editor)->connectClose([this]() {
				this->closeEditor();
			});
        }
        (*this->editor)->setValue(std::move(media));
        this->addWidget(*this->editor);
        this->setCurrentWidget(*this->editor);
    }

    void MainWidget::closeEditor() {
		this->removeWidget(*this->editor);
        this->editor.reset();
        this->setCurrentWidget(this->welcome);
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

        this->artistEdits = new XGWidget::MultiEdit;
        editorFormLayout->addRow("作者", this->artistEdits);

        this->album = new XGWidget::SingleEdit;
        editorFormLayout->addRow("专辑", this->album);

        this->genreEdits = new XGWidget::MultiEdit;
        editorFormLayout->addRow("流派", this->genreEdits);

        this->btns = new QDialogButtonBox(QDialogButtonBox::Close | QDialogButtonBox::Save,
                                          Qt::Orientation::Horizontal);
        connect(this->btns, &QDialogButtonBox::accepted, this, &Editor::save);
        mainLayout->addWidget(btns, 0, Qt::AlignBottom);

        this->setLayout(mainLayout);
    }

    void MainWidget::Editor::setValue(::rust::Box<XGLib::Media> &&media) {
		auto cover = media->front_cover();
		this->cover->setValue(std::move(media->front_cover()));

		auto title = media->title();
        this->title->setText(XGRust::toString(std::move(title)));

        auto artists = media->artists();
        this->artistEdits->setValues(XGRust::toListString(std::move(artists)));

        auto album = media->album();
        this->album->setValue(XGRust::toString(std::move(album)));

        auto geners = media->genres();
        this->genreEdits->setValues(XGRust::toListString(std::move(artists)));

		this->media.emplace(std::move(media));
    }

    void MainWidget::Editor::save() {
		if (not this->media) {
            QMessageBox::critical(this, "无法保存", "音频还未打开！");
            return ;
		}

        auto cover = this->cover->getValue();
        auto title = XGRust::fromString(std::move(this->title->text().trimmed()));
        auto artists = XGRust::fromListString(std::move(this->artistEdits->getValues()));
        auto album =
            XGRust::fromString(std::move(this->album->getValue()));
        auto genres =
            XGRust::fromListString(std::move(this->genreEdits->getValues()));

        XGLib::SaveTagData data = {
			.cover = cover,
			.title = title,
			.artists = artists,
			.album = album,
			.genres = genres
        };

		XGLib::saveAudioFile(*this->media, data);
    }

    void MainWidget::Editor::connectClose(std::function<void()> &&f) const {
		connect(this->btns, &QDialogButtonBox::rejected, this, f);
    }
}
