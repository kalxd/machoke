#include "coverhistory.h"

namespace XGApp {
	CoverHistory::CoverHistory(QWidget *parent) {
        this->dock = new QDockWidget("以往封面", parent);
        this->dock->setAllowedAreas(Qt::RightDockWidgetArea);
        this->dock->setFeatures(QDockWidget::NoDockWidgetFeatures);
	}
}
