const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: './src/web/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'js/index.js',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    extensions: ['.js', '.wasm'],
  },
  mode: 'development',
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: './www/index.html', to: './' },
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
      outDir: path.resolve(__dirname, 'pkg'),
    }),
  ],
  devServer: {
    static: './dist',
    hot: true,
  },
};