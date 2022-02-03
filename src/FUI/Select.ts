import * as m from "mithril";
import { Maybe, Nothing, Just } from "purify-ts/Maybe";
import IORef from "../prelude/IORef";
import { selectClass } from "./Interal/Util";
import { IOutter, Outter } from "./Outter";

export interface ISelect<T = string> {
	items?: Iterable<T>;
	placeholder?: string;
	onselect?: (item: T) => void;
}

interface ISelectMenu<T = string> {
	isShow: boolean;
	items?: Iterable<T>;
	onselect: (item: Maybe<T>) => void;
}

const SelectMenu: m.Component<ISelectMenu> = {
	view: (vnode: m.Vnode<ISelectMenu>) => {
		if (!vnode.attrs.isShow) {
			return null;
		}

		const outterOption: IOutter = {
			onclick: (_) => vnode.attrs.onselect(Nothing)
		};

		const option = {
			style: {
				dislay: "block !important"
			}
		};

		const items = [...(vnode.attrs.items ?? [])];
		return m(Outter, outterOption, m(
			"div.menu.transition.visible.animate__animated.animate__flipInX.animate__faster",
			option,
			items.map(text =>
				m(
					"div.item",
					{
						onclick: () => vnode.attrs.onselect(Just(text))
					},
					text
				)
			)
		));
	}
};

export const Select = (): m.Component<ISelect> => {
	const state = new IORef<boolean>(false);

	return {
		view: (vnode: m.Vnode<ISelect>) => {
			const outterOption: IOutter = {
				onclick: (_) => {
					state.write(false);
				}
			};

			const option = {
				class: selectClass({
					active: state.read(),
					visible: state.read(),
				}),

				onclick: () => {
					const b = state.read();
					state.write(!b);
				}
			};

			const menuOption: ISelectMenu = {
				isShow: state.read(),
				items: vnode.attrs.items,
				onselect: (value) => {
					value.ifJust(x => {
						if (vnode.attrs.onselect) {
							vnode.attrs.onselect(x);
						}
					});
				}
			};

			return m(Outter, outterOption, [
				m("div.ui.selection.dropdown", option, [
					m("i.icon.dropdown"),
					m("div.default.text", vnode.attrs.placeholder),
					m(SelectMenu, menuOption)
				])
			])
		}
	};
};
