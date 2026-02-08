#include "util.h"

namespace XGRust {
	QString toString(const String &&s) {
        return QString::fromStdString((std::string)s);
    }

    QList<QString> toListString(const Vec<String> &&s) {
		QList<QString> result;
        for (const auto item : s) {
            result << XGRust::toString(std::move(item));
        }

        return result;
    }

    String fromString(const QString &&s) { return String(s.toStdString()); }

    Vec<String> fromListString(const QList<QString> &&xs) {
		Vec<String> result;

        for (const auto x : xs) {
            result.push_back(XGRust::fromString(std::move(x)));
        }

        return result;
    }
}
