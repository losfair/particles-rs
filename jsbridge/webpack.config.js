const path = require("path");
const webpack = require("webpack");

let config = {
    entry: {
        bridge: "./src/bridge.ts"
    },
    output: {
        filename: "[name].js",
        path: path.join(__dirname, ".")
    },

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".ts", ".js", ".json"]
    },

    module: {
        rules: [
            // All files with a '.ts' or '.tsx' extension will be handled by 'awesome-typescript-loader'.
            { test: /\.ts$/, loader: "awesome-typescript-loader" },

            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" }
        ]
    },

    plugins: [
        new webpack.EnvironmentPlugin([
            "NODE_ENV",
            "PRS_BUILD_ID"
        ])
    ]
};

module.exports = config;
