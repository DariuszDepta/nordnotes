const CopyPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
  entry: {
    app: './src/index.ts'
  },
  output: {
    filename: 'nordnotes-100.js',
    path: path.resolve(__dirname, 'dist')
  },
  plugins: [
    new CopyPlugin({
        patterns: [
          {from: 'src/index.html'},
          {from: 'src/favicon.ico'},
        ],
        options: {
          concurrency: 100,
        }
      }
    ),
  ],
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.html$/,
        loader: 'html-loader'
      },
      {
        test: /\.ico$/,
        loader: 'file-loader'
      },
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.js']
  },
}
