import * as m from "mithril";

export interface IInputAttr {
	value?: string;
	placeholder?: string;
	onchange?: (e: Event) => void;
}

export const Input: m.Component<IInputAttr> = {
	view: (vnode: m.Vnode<IInputAttr>) => {
		return m("div.ui.input", [
			m("input", { ...vnode.attrs })
		])
	}
}
