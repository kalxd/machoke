const { R } = require("drifloon");

// tabId :: Maybe Tab
let tab = null;

// () -> Promise Tab
const createNewPage = () => {
	const option = {
		url: browser.runtime.getURL("./page.html")
	};
	browser.tabs.create(option)
		.then(tab_ => {
			tab = tab_;
			return tab_;
		});
};


const openPage = () => {
	if (tab) {
		return browser.tabs.query({})
			.then(R.find(R.eqProps("id", tab)))
			.then(R.ifElse(
				R.isNil,
				createNewPage,
				tab => browser.tabs.update(tab.id, { active: true, })
			));
	}
	else {
		return createNewPage();
	}
};

browser.browserAction.onClicked.addListener(openPage);
