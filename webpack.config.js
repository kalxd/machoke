const path = require("path");

const config = {
	target: "web",
	mode: "production",

	context: path.resolve("."),

	entry: {
		main: "./src/main.ts",
		background: "./src/background.ts"
	},

	output: {
		path: path.resolve("webextension/dist")
	},

	resolve: {
		extensions: [".ts", ".js"]
	},

	module: {
		rules: [
			{
				test: /\.ts$/,
				use: "ts-loader",
				exclude: /node_modules/
			}
		]
	}
};

module.exports = config;
