#ifndef XGWIDGET_MULTIEDIT
#define XGWIDGET_MULTIEDIT

#include <QComboBox>
#include <QVBoxLayout>
#include <QStringListModel>
#include <QPushButton>

namespace XGWidget {
	class MultiEdit : public QWidget {
    private:
		QComboBox *firstCombo;
        QVBoxLayout *expandLayout;
        QList<QComboBox*> expandBoxs;

        QStringListModel *model;
        QPushButton *addBtn;

        QComboBox *createCombox(QWidget *parent = nullptr) const;
        void addBlankLine();
    public:
		explicit MultiEdit(QWidget *parent = nullptr);
        void setValues(const QStringList &&xs);
        QList<QString> getValues() const;
	};
}

#endif
