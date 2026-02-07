#include "util.h"
#include <string>

namespace XGWidget {
	QString Rust::toString(const ::rust::String &&s) {
		return QString::fromStdString((std::string)s);
	}

	QList<QString> Rust::toListOfString(const ::rust::Vec<::rust::String> &&xs) {
		QList<QString> as;
        for (const auto x : xs) {
            as << Rust::toString(std::move(x));
        }

        return as;
	}
}
