import * as m from "mithril";
import { Just } from "purify-ts/Maybe";
import { Select, ISelectAttr } from "../../FUI/Select";
import { HttpMethod, httpMethods } from "../../prelude/Http";

export interface IMethodAttr {
	value: HttpMethod;
	onselect: (method: HttpMethod) => void;
}

export const MethodSelect: m.Component<IMethodAttr> = {
	view: (vnode: m.Vnode<IMethodAttr>) => {
		const selectAttr: ISelectAttr<HttpMethod> = {
			value: Just(vnode.attrs.value),
			items: httpMethods,
			onselect: vnode.attrs.onselect
		};

		return m<ISelectAttr<HttpMethod>, {}>(Select, selectAttr);
	}
}
