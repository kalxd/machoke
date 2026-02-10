#ifndef XGRUST_UTIL
#define XGRUST_UTIL

#include "lib.rs.h"
#include <QString>
#include <QList>
#include <QByteArray>

namespace XGRust {
	using namespace ::rust;

    QString toString(const String &&s);
    QList<QString> toListString(const Vec<String> &&);

	String fromString(const QString &&s);
    Vec<String> fromListString(const QList<QString> &&s);

    QByteArray toByteArray(const Vec<std::uint8_t> &&xs);
    Vec<std::uint8_t> fromByteArray(const QByteArray &xs);

    const char *toMimeString(const XGLib::CoverMime &mime);

	XGLib::CoverMime mimeFromString(const QString &mime);
}

#endif
