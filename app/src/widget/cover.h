#ifndef _XG_COVER_
#define _XG_COVER_

#include <QWidget>
#include <QGroupBox>

namespace XGWidget {
	class Cover : public QGroupBox {
    public:
        explicit Cover(QWidget* parent = nullptr);
	};
}

#endif
