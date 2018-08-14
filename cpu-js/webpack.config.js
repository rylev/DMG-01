const path = require('path')
module.exports = {
    entry: "./src/index.jsx",
    output: {
        filename: "cpu.js",
      path: __dirname + "/dist",
      // Export this as a library that can be accessed from plain JS
      libraryTarget: 'var',
      library: 'CPU'
    },

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".ts", ".tsx", ".js", ".json", ".wasm"]
    },
    mode: 'production',

    module: {
        rules: [
            // All files with a '.ts' or '.tsx' extension will be handled by 'awesome-typescript-loader'.
            // { test: /\.tsx?$/, loader: "awesome-typescript-loader" },

            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
          { enforce: "pre", test: /\.js$/, loader: "source-map-loader" },
          {
            test: /\.jsx?$/,
            exclude: /node_modules/,
            include: [ path.resolve(__dirname, 'src') ],
            use: {
              loader: 'babel-loader',
              options: {
                presets: ['es2015'],
                plugins: ["transform-react-jsx", "syntax-dynamic-import"]
              }
            }
          },
          {
            test:/\.css$/,
              use:['style-loader','css-loader']
          }
        ]
    },
  // When importing a module whose path matches one of the following, just
  // assume a corresponding global variable exists and use that instead.
  // This is important because it allows us to avoid bundling all of our
  // dependencies, which allows browsers to cache those libraries between builds.
  externals: {
    "react": "React",
    "react-dom": "ReactDOM"
  }

};
