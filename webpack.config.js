const MiniCssExtractPlugin = require("mini-css-extract-plugin");

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
			},
			{
				test: /\.css$/,
				use: [MiniCssExtractPlugin.loader, "css-loader"]
			}
		]
	},

	plugins: [new MiniCssExtractPlugin()]
};

module.exports = config;
