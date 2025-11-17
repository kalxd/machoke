#ifndef XGWIDGET_MULTI_INPUT
#define XGWIDGET_MULTI_INPUT

#include <QLineEdit>
#include <QVBoxLayout>
#include <QPushButton>

namespace XGWidget {
	class MultiInputEdit : public QWidget {
    private:
		QLineEdit *edit = new QLineEdit;;
        QPushButton *removeBtn = new QPushButton("删除");

    public:
        explicit MultiInputEdit(QWidget *parent = nullptr);
	};

    class MultiInput : public QVBoxLayout {
    private:
		QList<MultiInputEdit *> edits;
        QLineEdit *fixEdit = new QLineEdit;
        QLayout *editLayout = new QVBoxLayout;
        QPushButton *addBtn = new QPushButton("添加一列");

    public:
		explicit MultiInput(QWidget *parent = nullptr);
		~MultiInput();
	};
}

#endif
