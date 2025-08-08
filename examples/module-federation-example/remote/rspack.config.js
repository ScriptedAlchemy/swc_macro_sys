const rspack = require('@rspack/core');
const { ModuleFederationPlugin } = rspack.container;

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  target: 'async-node',
  devtool: false,
  output: {
    publicPath: 'http://localhost:3002/',
    library: {
      type: 'commonjs-module',
    },
  },
  devServer: {
    port: 3002,
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        use: {
          loader: 'builtin:swc-loader',
          options: {
            jsc: {
              parser: {
                syntax: 'ecmascript',
                jsx: true,
              },
              target: 'es2015',
            },
          },
        },
      },
    ],
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'remote',
      filename: 'remoteEntry.js',
      library: { type: 'commonjs-module' },
      exposes: {
        './Button': './src/Button',
        './utils': './src/utils',
        './dateUtils': './src/dateUtils',
        './functionalUtils': './src/functionalUtils',
      },
      shared: {
        'lodash-es': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.17.21',
        },
        'date-fns': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.1.0',
        },
        'ramda': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^0.31.3',
        },
      },
    }),
    new rspack.HtmlRspackPlugin({
      template: './src/index.html',
    }),
  ],
};