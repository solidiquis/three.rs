const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = (env, args) => {
  const isProductionMode = (args.mode === 'production');

  return {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: isProductionMode ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    experiments: {
      asyncWebAssembly: true,
    },
    devServer: {
      static: {
        directory: path.join(__dirname, 'public'),
        publicPath: '/public',
      },
      hot: true
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.'),
        outName: "three_rs"
      }),
      new webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder']
      }),
    ],
  };
}
