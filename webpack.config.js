const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (env, argv) => {
    return {
        entry: "./index.js",
        output: {
            path: path.resolve(__dirname, "dist"),
            filename: "index.js",
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, ".")
            }),
        ],
        experiments: {
            asyncWebAssembly: true
        },
        watch: argv.mode !== "production"
    };
};