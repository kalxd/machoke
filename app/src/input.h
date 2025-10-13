#ifndef _XG_INPUT_
#define _XG_INPUT_

#include <QLineEdit>
#include <QCompleter>
#include <QStringListModel>

namespace XGApp {
	class Input : public QLineEdit {
		Q_OBJECT
    private:
        bool isWordInModel(const QString& word) const;
        void appendToModel(const QString& word);
		void setup();

        QStringListModel* model = new QStringListModel(this);
        QCompleter* completer = new QCompleter(this->model, this);

    public:
		explicit Input(QWidget *parent = nullptr);
		QString text();
    };
}

#endif
