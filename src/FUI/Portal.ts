import * as m from "mithril";

export interface IPortal {
	node: HTMLElement;
}

export const Portal: m.Component<IPortal> = {
	oncreate: (vnode: m.VnodeDOM<IPortal>) => {
		const c = {
			view: () => vnode.children
		};
		m.mount(vnode.attrs.node, c);
	},

	onremove: (vnode: m.VnodeDOM<IPortal>) => {
		m.mount(vnode.attrs.node, null);
	},

	view: () => m.fragment({}, "")
};
