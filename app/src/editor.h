#ifndef _XG_EDITOR_
#define _XG_EDITOR_

#include <QWidget>

namespace XGApp {
	class Editor : public QWidget {
    public:
		explicit Editor(QWidget *parent = nullptr);

    private:
        void setup();
	};
}

#endif
