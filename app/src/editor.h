#ifndef _XG_EDITOR_
#define _XG_EDITOR_

#include <QWidget>
#include <QLineEdit>
#include "input.h"

namespace XGApp {
	class Editor : public QWidget {
		Q_OBJECT
    public:
		explicit Editor(QWidget *parent = nullptr);

    private:
        QWidget* setupBasicForm();
        void setup();

        XGApp::Input *titleLine;
	};
}

#endif
