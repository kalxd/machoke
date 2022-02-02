/**
 * 整个控制室
 */
import * as m from "mithril";
import {
	ControlGroup,
	Input,
	IInputAttrs,
	Select,
	ISelectAttrs,
	Button
} from "construct-ui";

import { httpMethods } from "../prelude/Http";
import IORef from "../prelude/IORef";

const selectOption: Readonly<ISelectAttrs> = {
	options: [...httpMethods]
};

const Main = (): m.ClassComponent => {
	const state = new IORef<string>("hello world")
	return {
		view: () => {
			const inputOption: IInputAttrs = {
				value: state.read(),
				placeholder: "来这里输入",
				onchange: e => {
					let value = (e.target as HTMLInputElement)?.value;
					state.write(value);
				}
			};

			return m(ControlGroup, [
				m(Select, selectOption),
				m(Input, inputOption),
				m(Button, { label: "来一砲"})
			]);
		}
	};
};

export default Main;
