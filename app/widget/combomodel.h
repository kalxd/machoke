#ifndef XGWIDGET_COMBOMODEL
#define XGWIDGET_COMBOMODEL

#include <QStringListModel>

namespace XGWidget {
	class ComboModel : public QStringListModel {

    public:
		explicit ComboModel(QObject *parent = nullptr);

        void appendWord(const QString &word);
        void appendWords(const QList<QString> &words);
	};
}

#endif
