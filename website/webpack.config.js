const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const crateDir = path.resolve(__dirname, "..");

module.exports = {
  mode: process.env.NODE_ENV || "development",
  entry: "./src/bootstrap.mjs",
  target: "web",
  output: {
    path: path.resolve(__dirname, "dist/script"),
    publicPath: "/script/",
    filename: "[name].js",
  },

  module: {
    rules: [
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.scss$/,
        use: ["style-loader", "css-loader", "sass-loader"],
      },
      {
        test: /\.wasm$/,
        include: path.resolve(__dirname, "src"),
        use: "wasm-loader",
      },
    ],
  },

  experiments: {
    syncWebAssembly: true,
    topLevelAwait: true,
  },

  plugins: [
    new WasmPackPlugin({
      outName: "osrs-cli",
      crateDirectory: crateDir,
      watchDirectories: [
        path.resolve(crateDir, "Cargo.toml"),
        path.resolve(crateDir, "src"),
      ],
      outDir: path.resolve(__dirname, "wasm"),
    }),
  ],

  resolve: {
    modules: ["node_modules"],
    extensions: [".js"],
  },

  watchOptions: {
    ignored: /node_modules/,
  },
};
