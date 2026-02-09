#ifndef XGWIDGET_MULTIEDIT
#define XGWIDGET_MULTIEDIT

#include <QComboBox>
#include <QVBoxLayout>
#include <QStringListModel>
#include <QPushButton>
#include <functional>

namespace XGWidget {
	namespace {
		static QComboBox* createCombox(QAbstractListModel *model, QWidget *parent = nullptr);
	}

	class MultiEdit : public QWidget {
    private:
		class EditRow;

        QComboBox *firstCombo;
        QVBoxLayout *expandLayout;
        QList<EditRow*> expandBoxs;

        QStringListModel *model;
        QPushButton *addBtn;

        std::optional<QModelIndex> tryInsertModel(const QString &word);
        void setModelStrings(const QStringList &s);
        void addBlankLine(const QString &&init);
    public:
		explicit MultiEdit(QWidget *parent = nullptr);
        void setValues(const QStringList &&xs);
        QList<QString> getValues();
    };

    class MultiEdit::EditRow : public QWidget {
    private:
		QComboBox *combo;
		QPushButton *removeBtn;
    public:
		explicit EditRow(QAbstractListModel *model, QWidget *parent = nullptr);
        QString getValue() const;
        void setValue(const QString &&s);

        void connectRemove(std::function<void()> f) const;
    };
}

#endif
