#ifndef XGWIDGET_SINGLEEDIT
#define XGWIDGET_SINGLEEDIT

#include <QComboBox>
#include "combomodel.h"

namespace XGWidget {
	class SingleEdit : public QComboBox {
    private:
        ComboModel *model;
    public:
		explicit SingleEdit(QWidget *parent = nullptr);

        QString getValue();
        void setValue(const QString &word);
	};
}

#endif
