const path = require("path");

const config = {
	target: "web",
	mode: "production",

	context: path.resolve("."),

	entry: {
		main: "./src/main.js",
		background: "./src/background.js"
	},

	output: {
		path: path.resolve("webextension/dist")
	}
};

module.exports = config;
