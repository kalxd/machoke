#ifndef XGWIDGET_COVER
#define XGWIDGET_COVER

#include <QGroupBox>
#include <QLabel>

namespace XGWidget {
	class Cover : public QGroupBox {
    private:
		QLabel *coverLabel;
    public:
        explicit Cover(QWidget *parent = nullptr);
	};
}

#endif
