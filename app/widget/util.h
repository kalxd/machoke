#ifndef XGWIDGET_UTIL
#define XGWIDGET_UTIL

#include "lib.rs.h"
#include <QList>
#include <QString>

namespace XGWidget {
	class Rust {
	public:
		static QString toString(const ::rust::String &&s);
        static QList<QString> toListOfString(const ::rust::Vec<::rust::String> &&xs);

        Rust() = delete;
        Rust(Rust &) = delete;

        ~Rust() = delete;
    };

}

#endif
