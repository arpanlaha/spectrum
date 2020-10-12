/* eslint-disable @typescript-eslint/no-var-requires */

const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/startup.ts",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "startup.js",
  },
  mode: "production",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "src/index.html", to: "." },
        { from: "static", to: "static" },
      ],
    }),
  ],
};
