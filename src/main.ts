import * as m from "mithril";

import Container from "./widget/container";
import Sidebar from "./layout/sidebar";

const App: m.ClassComponent = {
	view: () => m(Container, m("div.ui.grid", [
		m("div.six.wide.column", [
			m(Sidebar)
		]),
		m("div.ten.wide.column", m("div.ui.blue.segment", [
			m("h1", "hello world")
		]))
	]))
};

m.mount(document.body, App)
