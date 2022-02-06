import * as m from "mithril";
import { Either } from "purify-ts/Either";
import { Maybe, Nothing, Just } from "purify-ts/Maybe";
import IORef from "../prelude/IORef";
import { selectClass } from "./Interal/Util";
import { IOutter, Outter } from "./Outter";

export interface ISelectAttr<T = string> {
	value: Maybe<T>;
	items: Iterable<T>;
	placeholder?: string;
	onselect?: (item: T) => void;
	render?: (item: T) => string;
}

interface ISelectMenuAttr<T = string> {
	isShow: boolean;
	items: Iterable<T>;
	onselect: (item: Maybe<T>) => void;
	render: (item: T) => string;
}

const SelectMenu = <T>(): m.Component<ISelectMenuAttr<T>> => {
	return {
		view: (vnode: m.Vnode<ISelectMenuAttr<T>>) => {
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

			const items = [...vnode.attrs.items];
			return m(Outter, outterOption, m(
				"div.menu.transition.visible.animate__animated.animate__flipInX.animate__faster",
				option,
				items.map(text =>
					m(
						"div.item",
						{
							onclick: () => vnode.attrs.onselect(Just(text))
						},
						vnode.attrs.render(text)
					)
				)
			));
		}
	};
};

const renderText = (text: Either<string, string>): m.Vnode => text.bimap(
	text => m("div.default.text", text),
	text => m("div.text", text)
).extract();

export const Select = <T>(): m.Component<ISelectAttr<T>> => {
	const state = new IORef<boolean>(false);

	return {
		view: (vnode: m.Vnode<ISelectAttr<T>>) => {
			const render = vnode.attrs.render ?? String;

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

			const menuOption: ISelectMenuAttr<T> = {
				isShow: state.read(),
				items: vnode.attrs.items,
				render,
				onselect: value => {
					value.ifJust(x => {
						if (vnode.attrs.onselect) {
							vnode.attrs.onselect(x);
						}
					});
				}
			};

			const text = vnode.attrs.value.toEither(vnode.attrs.placeholder ?? "").map(render);

			return m(Outter, outterOption, [
				m("div.ui.selection.dropdown", option, [
					m("i.icon.dropdown"),
					renderText(text),
					m<ISelectMenuAttr<T>, {}>(SelectMenu, menuOption)
				])
			])
		}
	};
};
