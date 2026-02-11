#include "coverhistory.h"
#include "rust/util.h"
#include <QList>
#include <QLayout>
#include <qabstractitemmodel.h>

namespace XGApp {
	CoverHistory::CoverHistory(QWidget *parent) : QDockWidget(parent) {
		this->setWindowTitle("封面");
        this->setAllowedAreas(Qt::RightDockWidgetArea);
        this->setFeatures(QDockWidget::NoDockWidgetFeatures);
        this->setMinimumWidth(200);

        this->model = new CoverModel;

        this->view = new QListView;
        this->view->setViewMode(QListView::IconMode);
        this->view->setIconSize(QSize(125, 125));
        this->view->setModel(this->model);

        this->setWidget(this->view);
    }

    void CoverHistory::appendCover(const XGRust::CoverInfo &&info) {
		this->model->appendValue(std::move(info));
    }

    void CoverHistory::connectChoose(const std::function<void(XGRust::CoverInfo &&)> &&f) const {
      connect(this->view, &QListView::doubleClicked, this,
              [this, f](const auto index) {
                  auto v = this->model->getValue(index);
                  f(std::move(v));
		});
    }

    CoverHistory::CoverModel::CoverModel(QObject *parent)
    : QAbstractListModel(parent) {}

    int CoverHistory::CoverModel::rowCount(const QModelIndex &) const {
        return this->covers.length();
    }

    QVariant CoverHistory::CoverModel::data(const QModelIndex &index,
                                            int role) const {
        if (role == Qt::DecorationRole) {
			return QIcon(this->covers.at(index.row()).pixmap);
        }

        return {};
    }

    void CoverHistory::CoverModel::appendValue(const XGRust::CoverInfo &&info) {
		for (const auto &item : this->covers) {
            if (item.path == info.path) {
                return ;
            }
		}

		auto rawTotal = this->covers.length();

        this->beginInsertRows(QModelIndex(), rawTotal, rawTotal);
        this->covers << std::move(info);
        this->endInsertRows();
    }

    XGRust::CoverInfo
    CoverHistory::CoverModel::getValue(const QModelIndex &index) const {
		return this->covers.at(index.row());
    }
}
