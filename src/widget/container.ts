import * as m from "mithril";

const Container: m.ClassComponent = {
	view: (vnode: m.Vnode) => m("div.ui.fluid.container.basic.very.padded.segment", vnode.children)
}

export default Container;
