import { tabs, runtime, browserAction, Tabs } from "webextension-polyfill";
import { IORef, newIORef } from "fp-ts/IORef";
import * as Option from "fp-ts/Option";
import * as Task  from "fp-ts/Task";
import * as IO from "fp-ts/IO";
import * as IArray from "fp-ts/Array";
import { pipe, flow, constVoid } from "fp-ts/function";

const tabID: (tab: Tabs.Tab) => Option.Option<number> = flow(
	tab => tab.id,
	Option.fromNullable
);

const tabRef: IO.IO<IORef<Option.Option<Tabs.Tab>>> = newIORef(Option.none);

const baseTabOption: Tabs.CreateCreatePropertiesType = {
	url: runtime.getURL("./page.html")
};

const createTab: (a: Tabs.CreateCreatePropertiesType) => Task.Task<Tabs.Tab> =
	option => () => tabs.create(option);

const createTabAndSet: Task.Task<void> = pipe(
	createTab(baseTabOption),
	Task.chain(tab => pipe(
		tabRef,
		IO.chain(ref => ref.write(Option.some(tab))),
		Task.fromIO
	))
);

const activeTab: (a: number) => Task.Task<void> = id => pipe(
	() => tabs.update(id, { active: true }),
	Task.map(constVoid)
);

const activeTabById: (a: number) => Task.Task<void> = id => pipe(
	() => tabs.query({}),
	Task.map(IArray.findFirst(tab => tab.id == id)),
	Task.map(Option.chain(tabID)),
	Task.chain(Option.fold(
		() => () => Promise.resolve(),
		activeTab
	))
);

const openPage: Task.Task<void> = pipe(
	tabRef,
	IO.chain(ref => ref.read),
	IO.map(Option.chain(tabID)),
	Task.fromIO,
	Task.chain(Option.fold(
		() => createTabAndSet,
		activeTabById
	))
);

browserAction.onClicked.addListener(openPage);
