const rspack = require('@rspack/core');
const { ModuleFederationPlugin } = rspack.container;
const isProd = process.env.NODE_ENV === 'production';

module.exports = {
  mode: isProd ? 'production' : 'development',
  entry: './src/index.js',
  target: 'web',
  devtool: false,
  output: {
    publicPath: 'http://localhost:3001/',
    clean: true,
  },
  devServer: {
    port: 3001,
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
      name: 'host',
      remotes: {
        remote: 'remote@http://localhost:3002/remoteEntry.js',
      },
      shared: {
        'lodash-es': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.17.21',
          eager: false,
        },
      },
    }),
    new rspack.HtmlRspackPlugin({
      template: './src/index.html',
    }),
  ],
};