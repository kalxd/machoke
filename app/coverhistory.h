#ifndef XGAPP_COVERHISTORY
#define XGAPP_COVERHISTORY

#include "rust/util.h"
#include <QDockWidget>
#include <QAbstractListModel>
#include <QListView>
#include <functional>

namespace XGApp {
	class CoverHistory : public QDockWidget {
    private:
		class CoverModel;

        CoverModel *model;
        QListView *view;
    public:
		explicit CoverHistory(QWidget *parent = nullptr);

        void appendCover(const XGRust::CoverInfo &&info);
        void connectChoose(const std::function<void(XGRust::CoverInfo &&)> &&f) const;
    };

    class CoverHistory::CoverModel : public QAbstractListModel {
    private:
		QList<XGRust::CoverInfo> covers;
    public:
		explicit CoverModel(QObject *parent = nullptr);

        int rowCount(const QModelIndex &parent = QModelIndex()) const final override;
        QVariant data(const QModelIndex &index, int role = Qt::DecorationRole) const final override;

        void appendValue(const XGRust::CoverInfo &&info);
        XGRust::CoverInfo getValue(const QModelIndex &index) const;
    };
}

#endif
