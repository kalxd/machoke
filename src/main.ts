import * as m from "mithril";
import "construct-ui/lib/index.css";

import { Grid, IGridAttrs, Col, IColAttrs } from "construct-ui";

const gridOption: Readonly<IGridAttrs> = {
	gutter: 20
};

const withColOption = (span: number): Readonly<IColAttrs> => ({ span });

const App: m.ClassComponent = {
	view: () => m(Grid, gridOption, [
		m(Col, withColOption(4), "left"),
		m(Col, withColOption(8), "Right")
	])
};

m.mount(document.body, App)
