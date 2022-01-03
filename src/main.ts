import * as m from "mithril";

import Container from "./widget/container";

import Sidebar from "./layout/sidebar";

const App: m.ClassComponent = {
	view: () => m(Container, [
		m(Sidebar)
	])
};

m.mount(document.body, App)
