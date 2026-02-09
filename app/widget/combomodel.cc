#include "combomodel.h"

namespace XGWidget {
	ComboModel::ComboModel(QObject *parent) : QStringListModel(parent) {}

    void ComboModel::appendWord(const QString &word) {
		auto total = this->rowCount();

        for (int i = 0; i < total; ++i) {
			auto index = this->index(i);
            auto value = this->data(index).toString();
            if (value == word) {
				return ;
            }
        }

        if (this->insertRow(total)) {
			auto index = this->index(total);
            this->setData(index, word);
        }
    }

    void ComboModel::appendWords(const QList<QString> &words) {
		for (const auto x : words) {
			this->appendWord(x);
		}
    }
}
