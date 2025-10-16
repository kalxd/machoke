#ifndef _XG_COVER_
#define _XG_COVER_

#include <QWidget>
#include <QGroupBox>
#include <QLabel>

namespace XGWidget {
	class Cover : public QGroupBox {
		Q_OBJECT
    private:
		QLabel *imageLabel;

        void setup();

    public:
        explicit Cover(QWidget* parent = nullptr);
	};
}

#endif
