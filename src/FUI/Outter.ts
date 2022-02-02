import * as m from "mithril";

export interface IOutter {
	onclick?: (event: MouseEvent) => void;
}

export const Outter = (): m.Component<IOutter> => {
	let innerBinding: ((e: MouseEvent) => void) | null = null;

	return {
		oninit: (vnode: m.Vnode<IOutter>) => {
			innerBinding = (e: MouseEvent): void => {
				const children = vnode.children as Array<m.VnodeDOM> ?? [];
				const el = e.target as HTMLElement;

				if (el !== null) {
					const isInner = children.some(child => child.dom.contains(el));
					if (!isInner && vnode.attrs.onclick) {
						vnode.attrs.onclick(e);
						m.redraw();
					}
				}
			};

			document.body.addEventListener("click", innerBinding);
		},

		onbeforeremove: () => {
			if (innerBinding) {
				document.body.removeEventListener("click", innerBinding);
			}
		},

		view: (vnode: m.Vnode) => {
			return vnode.children;
		}
	};
};
