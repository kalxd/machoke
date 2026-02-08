#ifndef XGWIDGET_COVER
#define XGWIDGET_COVER

#include <QGroupBox>
#include <QLabel>
#include <QPixmap>
#include <optional>

namespace XGWidget {
	class Cover : public QGroupBox {
    private:
		QLabel *coverLabel;
        std::optional<QPixmap *> pixmap = std::nullopt;

        void chooseCover();

    public:
        explicit Cover(QWidget *parent = nullptr);
	};
}

#endif
