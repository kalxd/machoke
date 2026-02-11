#ifndef XGWIDGET_COVER
#define XGWIDGET_COVER

#include "lib.rs.h"
#include "../rust/util.h"
#include <QGroupBox>
#include <QLabel>
#include <QPixmap>
#include <optional>

namespace XGWidget {
	class Cover : public QGroupBox {
		Q_OBJECT
    private:
		QLabel *coverLabel;
        std::optional<const QPixmap *> pixmap = std::nullopt;
        XGLib::CoverMime mime = XGLib::CoverMime::None;

        void loadPixmap(const QPixmap *pixmap);
        void chooseCover();
        void removeCover();
	signals:
        void updateCover(const XGRust::CoverInfo info);

    public:
		explicit Cover(QWidget *parent = nullptr);

        void setValue(const ::rust::Box<XGLib::CoverTuple> &&cover);
        XGLib::CoverTuple getValue() const;

        void setCover(const XGRust::CoverInfo &&info);
	};
}

#endif
