/**
 * 最主要的界面
 */
import * as m from "mithril";
import { UrlEditor } from "./Component/UrlEditor";

const Main: m.Component = {
	view: () => {
		return m("div.ui.container.teai.segment", [
			m("h1", "hello world"),
			m(UrlEditor)
		]);
	}
};

export default Main;
