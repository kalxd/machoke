#ifndef XGWIDGET_COVER
#define XGWIDGET_COVER

#include "lib.rs.h"
#include <QGroupBox>
#include <QLabel>
#include <QPixmap>
#include <optional>

namespace XGWidget {
	class Cover : public QGroupBox {
    private:
		QLabel *coverLabel;
        std::optional<const QPixmap *> pixmap = std::nullopt;

        void loadPixmap(const QPixmap *pixmap);
        void chooseCover();

    public:
		explicit Cover(QWidget *parent = nullptr);

        void setValue(const ::rust::Box<XGLib::CoverTuple> &&cover);
	};
}

#endif
