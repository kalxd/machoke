#include "coverhistory.h"

namespace XGApp {
	CoverHistory::CoverHistory(QWidget *parent) : QDockWidget(parent) {
		this->setWindowTitle("封面");
        this->setAllowedAreas(Qt::RightDockWidgetArea);
        this->setFeatures(QDockWidget::NoDockWidgetFeatures);
	}
}
