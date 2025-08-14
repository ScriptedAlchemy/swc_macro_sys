const rspack = require('@rspack/core');
const { ModuleFederationPlugin } = rspack.container;
const isProd = process.env.NODE_ENV === 'production';

module.exports = {
  mode: isProd ? 'production' : 'development',
  entry: './src/index.js',
  target: 'web',
  devtool: false,
  output: {
    publicPath: 'http://localhost:3002/',
    clean: true,
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
  optimization: {
    minimizer: isProd
      ? [
          new rspack.SwcJsMinimizerRspackPlugin({
            minimizerOptions: {
              mangle: false,
              compress: { passes: 2 },
              format: { comments: false }
            }
          })
        ]
      : []
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'remote',
      filename: 'remoteEntry.js',
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
          eager: false,
        },
        'date-fns': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.1.0',
          eager: false,
        },
        'ramda': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^0.31.3',
          eager: false,
        },
      },
    }),
    new rspack.HtmlRspackPlugin({
      template: './src/index.html',
    }),
  ],
};