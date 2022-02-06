import * as m from "mithril";
import { HttpMethod } from "../../prelude/Http";
import IORef from "../../prelude/IORef";
import { IMethodAttr, MethodSelect } from "../Widget/MethodSelect";

export const UrlEditor = (): m.Component => {
	const state = new IORef<HttpMethod>("GET");

	return {
		view: () => {
			const methodAttr: IMethodAttr = {
				value: state.read(),
				onselect: x => state.write(x)
			};

			return m(MethodSelect, methodAttr);
		}
	};
};
