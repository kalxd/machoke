#ifndef XGWIDGET_MULTI_INPUT
#define XGWIDGET_MULTI_INPUT

#include <QLineEdit>
#include <QVBoxLayout>
#include <QPushButton>
#include <vector>
#include <string>

namespace XGWidget {
	class MultiInputEdit : public QWidget {
		Q_OBJECT
    private:
		QLineEdit *edit = new QLineEdit;;
        QPushButton *removeBtn = new QPushButton("删除");

    public:
		explicit MultiInputEdit(QWidget *parent = nullptr);
        QString text() const;

	signals:
        void remove();
	};

    class MultiInput : public QWidget {
        Q_OBJECT
    private:
		QList<MultiInputEdit *> edits;
        QLineEdit *fixEdit = new QLineEdit;
        QLayout *editLayout = new QVBoxLayout;
        QPushButton *addBtn = new QPushButton("添加一列");

    private slots:
		void addEditLine();
		void removeEditLine(MultiInputEdit *edit);

    public:
		explicit MultiInput(QWidget *parent = nullptr);
        ~MultiInput();

        std::vector<std::string> texts() const;
	};
}

#endif
