#ifndef _XG_INPUT_
#define _XG_INPUT_

#include <QLineEdit>
#include <QCompleter>
#include <QStringListModel>

namespace XGApp {
	class Input : public QLineEdit {
		Q_OBJECT
    public:
		explicit Input(QWidget *parent = nullptr);

    private:
		void setup();
        QStringListModel* model = new QStringListModel(this);
        QCompleter* completer = new QCompleter(this->model, this);

	};
}

#endif
