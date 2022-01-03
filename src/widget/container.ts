import * as m from "mithril";

const Container: m.ClassComponent = {
	view: (vnode: m.Vnode) => m("div.ui.container", vnode.children)
}

export default Container;
