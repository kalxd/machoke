import * as m from "mithril";

const App = {
	view: () => {
		return m("main", [
			m("label", "hello")
		]);
	}
};

m.mount(document.body, App)
