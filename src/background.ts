import { Right, Left } from "purify-ts/Either";
import { EitherAsync } from "purify-ts/EitherAsync";
import { List } from "purify-ts/List";
import { Just, Maybe, Nothing } from "purify-ts/Maybe";
import { tabs, runtime, browserAction, Tabs } from "webextension-polyfill";
import IORef from "./prelude/IORef";

const tabRef: IORef<Maybe<Tabs.Tab>> = new IORef(Nothing);

type Task<T> = EitherAsync<Error, T>;

const tabID = (tab: Tabs.Tab): Maybe<number> => {
	const { id } = tab;
	return Maybe.fromNullable(id);
};

const baseTabOption: Tabs.CreateCreatePropertiesType = {
	url: runtime.getURL("./page.html")
};

const createTab = (option: Tabs.CreateCreatePropertiesType): Task<Tabs.Tab> =>
	EitherAsync.fromPromise(() => tabs.create(option).then(Right).catch(Left));

const createTabAndActive: Task<Tabs.Tab> =
	createTab(baseTabOption).ifRight(tab => tabRef.write(Just(tab)));


const activeTab = (id: number): Task<Tabs.Tab> =>
	EitherAsync.fromPromise(
		() => tabs.update(id, { active: true })
			.then(Right)
			.catch(Left)
	);

const queryAllTab: Task<Array<Tabs.Tab>> =
	EitherAsync.fromPromise(
		() => tabs.query({}).then(Right).catch(Left)
	);

const activeTabById = (id: number): Task<Tabs.Tab> => {
	return queryAllTab.chain(tabs => {
		return List.find(t => t.id === id, tabs)
			.chain(tab => Maybe.fromNullable(tab.id))
			.map(activeTab)
			.orDefault(createTabAndActive)
	});
};

const openPage = (): void => {
	tabRef.read().chain(tabID)
		.map(activeTabById)
		.orDefault(createTabAndActive)
		.run();
}

browserAction.onClicked.addListener(openPage);
