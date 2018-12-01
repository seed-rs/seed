const path = require('path');

module.exports = {
    entry: path.join(__dirname, 'main.tsx'),
    output: {
        filename: 'build/main.js',
        path: __dirname
    },
    module: {
        rules: [
            {
                exclude: /node_modules/,
            },
        ],

    },
    resolve: {
        extensions: [".tsx", ".ts", ".js", ".wasm"]
    },
    mode: "development"
};