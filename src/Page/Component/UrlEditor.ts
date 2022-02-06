import * as m from "mithril";
import IORef from "../../FUI/Interal/IORef";
import { HttpMethod } from "../../prelude/Http";
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
