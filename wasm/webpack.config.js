const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/app/app.component.ts",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "./src/app/app.component.ts",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new CopyPlugin({
      patterns: [{ from: "./src/app/app.component.html" },
      ],
    }),
  ],
};
