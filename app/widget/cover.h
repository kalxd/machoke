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
        XGLib::CoverMime mime = XGLib::CoverMime::None;

        void loadPixmap(const QPixmap *pixmap);
        void chooseCover();
        void removeCover();

    public:
		explicit Cover(QWidget *parent = nullptr);

        void setValue(const ::rust::Box<XGLib::CoverTuple> &&cover);
	};
}

#endif
