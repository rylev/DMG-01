const path = require('path')

module.exports = {
    entry: "./src/main.tsx",
    mode: "development",
    output: {
        filename: "bundle.js",
        path: __dirname + "/dist"
    },

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    resolve: {
        modules: [path.resolve(__dirname, "src"), "node_modules"],
        extensions: [".ts", ".tsx", ".js", ".json", ".wasm"]
    },

    module: {
      rules: [
        { test: /\.tsx?$/, use: "ts-loader" },
        { enforce: "pre", test: /\.js$/, use: "source-map-loader" },
        {
          test: /\.js$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: {
              plugins: [require("@babel/plugin-syntax-dynamic-import")]
            }
          },
        }
        ]
    }
};

