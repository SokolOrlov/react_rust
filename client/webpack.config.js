const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry:{
    main:'./src/App.tsx'
  },
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'dist'),
    clean: true,
  },
  mode:"development",
  module: {
    rules: [
      {test: /\.tsx?$/,use: 'ts-loader',exclude: /node_modules/},
      { test: /\.css$/, use: [
        { loader: 'style-loader'},
        {loader: 'css-loader', options: { modules: true }}] 
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  devServer: {
    static: './dist',
  },
   plugins: [
     new HtmlWebpackPlugin({template: './src/index.html'}),
   ],
};