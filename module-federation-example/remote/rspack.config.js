const rspack = require('@rspack/core');
const { ModuleFederationPlugin } = rspack.container;

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    publicPath: 'http://localhost:3002/',
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
      exposes: {
        './Button': './src/Button',
        './utils': './src/utils',
      },
      shared: {
        'lodash-es': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.17.21',
        },
      },
    }),
    new rspack.HtmlRspackPlugin({
      template: './src/index.html',
    }),
  ],
};