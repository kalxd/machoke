#include "multiedit.h"
#include <QHBoxLayout>
#include <qabstractitemmodel.h>

namespace XGWidget {
	namespace {
		QComboBox* createCombox(QAbstractListModel *model, QWidget *parent) {
			auto combo = new QComboBox(parent);
			combo->setModel(model);
			combo->setEditable(true);
			combo->setInsertPolicy(QComboBox::InsertAtTop);
			combo->setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Preferred);
			combo->setEditText("");
			return combo;
		}
	}

    MultiEdit::MultiEdit(QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QVBoxLayout;
        mainLayout->setContentsMargins(0, 0, 0, 0);

        this->model = new QStringListModel;

        this->firstCombo = createCombox(this->model, parent);
        mainLayout->addWidget(this->firstCombo);

        this->expandLayout = new QVBoxLayout;
        mainLayout->addLayout(this->expandLayout);

        this->addBtn = new QPushButton("添加新列");
        connect(this->addBtn, &QPushButton::clicked, this,
                [this]() { this->addBlankLine(""); });
        mainLayout->addWidget(this->addBtn);

        this->setLayout(mainLayout);
    }

    std::optional<QModelIndex> MultiEdit::tryInsertModel(const QString &word) {
		auto total = this->model->rowCount();

        for (int i = 0; i < total; ++i) {
			auto index = this->model->index(i);
            auto value = this->model->data(index).toString();

            if (value == word) {
				return {};
            }
        }

        if (this->model->insertRow(total)) {
			auto index = this->model->index(total);
            this->model->setData(index, word);

            return index;
        }

        return {};
    }

    void MultiEdit::setModelStrings(const QStringList &s) {
		for (const auto x : s) {
            auto index = this->tryInsertModel(x);
            if (index) {
				this->model->setData(*index, x);
            }
		}
    }

    void MultiEdit::addBlankLine(const QString &&init) {
		auto row = new EditRow(this->model);
		row->setValue(std::move(init));
        this->expandBoxs << row;
        this->expandLayout->addWidget(row);

        row->connectRemove([this, row]() {
			this->expandLayout->removeWidget(row);
			this->expandBoxs.removeIf(
									  [row](const EditRow *box) { return box == row; });
			delete row;
        });
    }

    void MultiEdit::setValues(const QStringList &&xs) {
		this->setModelStrings(xs);

        for (const auto item : this->expandBoxs) {
			this->expandLayout->removeWidget(item);
			delete item;
        }

        this->expandBoxs.clear();

        auto iter = xs.cbegin();
        if (iter == xs.cend()) {
			this->firstCombo->setEditText("");
            return;
        }

        this->firstCombo->setEditText(*iter);
        ++iter;

        while (iter != xs.cend()) {
            this->addBlankLine(std::move(*iter));
            ++iter;
        }
    }

    QList<QString> MultiEdit::getValues() {
		QList<QString> result;

        auto firstValue = this->firstCombo->currentText();
        if (not firstValue.isEmpty()) {
			result << firstValue;
        }

        for (const auto &item : this->expandBoxs) {
			auto value = item->getValue();
            if (not value.isEmpty()) {
                result << value;
            }
        }

        this->setModelStrings(result);

        return result;
    }

	MultiEdit::EditRow::EditRow(QAbstractListModel *model, QWidget *parent) : QWidget(parent) {
        auto mainLayout = new QHBoxLayout;
        mainLayout->setContentsMargins(0, 0, 0, 0);

        this->combo = createCombox(model, parent);
        mainLayout->addWidget(this->combo, 1);

        this->removeBtn = new QPushButton("删除");
        mainLayout->addWidget(this->removeBtn);

        this->setLayout(mainLayout);
    }

    QString MultiEdit::EditRow::getValue() const {
		return this->combo->currentText();
    }

    void MultiEdit::EditRow::setValue(const QString &&s) {
		this->combo->setEditText(s);
    }

    void
    MultiEdit::EditRow::connectRemove(std::function<void()> f) const {
		connect(this->removeBtn, &QPushButton::clicked, this, f);
    }
}
