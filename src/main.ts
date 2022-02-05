import * as m from "mithril";
import { Input } from "./FUI/Input";
import { Outter } from "./FUI/Outter";
import { Select } from "./FUI/Select";

const selectOption = {
	placeholder: "来挑选一个",
	items: [
		"one 1",
		"trwo 2"
	]
};

const App: m.ClassComponent = {
	view: () => m("div.ui.container.segment", [
		m("h1", "hello world"),
		m(Outter, [
			m(Select, selectOption),
			m("button", "hello first world"),
			m(Input)
		]),
		m(Input, { placeholder: "sb" })
	])
};

const appNode = document.createElement("main");
document.body.appendChild(appNode);

m.mount(appNode, App)
