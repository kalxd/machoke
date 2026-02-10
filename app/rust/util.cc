#include "util.h"
#include "lib.rs.h"

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

    QByteArray toByteArray(const Vec<std::uint8_t> &&xs) {
		QByteArray result;
        for (const auto x : xs) {
            result.append(x);
        }

        return result;
    }

    const char* toMimeString(const XGLib::CoverMime &mime) {
		if (mime == XGLib::CoverMime::Png) {
			return "PNG";
        } else {
            return "JPEG";
        }
    }

    XGLib::CoverMime mimeFromString(const QString &mime) {
		auto value = mime.toLower();
        if (value == "png") {
            return XGLib::CoverMime::Png;
        } else if (value == "jpg" or value == "jpeg") {
			return XGLib::CoverMime::Jpg;
        } else {
			return XGLib::CoverMime::None;
        }
    }
}
