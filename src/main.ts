import * as m from "mithril";
import { Input } from "./FUI/Input";
import { Outter } from "./FUI/Outter";

const App: m.ClassComponent = {
	view: () => m("div.ui.container.grid.segment", [
		m("h1", "hello world"),
		m(Outter, [
			m("button", "hello first world"),
			m(Input)
		]),
		m(Input, { placeholder: "sb" })
	])
};

m.mount(document.body, App)
