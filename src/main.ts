import * as m from "mithril";

import Controller from "./Page/Controller";

const App: m.ClassComponent = {
	view: () => m(Controller)
};

const appNode = document.createElement("main");
document.body.appendChild(appNode);

m.mount(appNode, App)
