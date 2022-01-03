import * as m from "mithril";

const Sidebar: m.Component = {
	view: () => m("aside.ui.vertical.menu", [
		// search
		m("div.item", [
			m("div.ui.transparent.icon.input", [
				m("input"),
				m("i.search.icon")
			])
		])
	])
};

export default Sidebar;
