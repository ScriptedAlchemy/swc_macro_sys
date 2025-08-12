import { defineConfig } from '@rspack/cli';
import { rspack } from '@rspack/core';
// import refreshPlugin from '@rspack/plugin-react-refresh';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const isDev = process.env.NODE_ENV === 'development';

export default defineConfig({
  context: __dirname,
  entry: './src/index.js',
  target: 'web',
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx', '.json']
  },
  module: {
    rules: [
      {
        test: /\.(jsx?|tsx?)$/,
        use: [
          {
            loader: 'builtin:swc-loader',
            options: {
              jsc: {
                parser: {
                  syntax: 'ecmascript',
                  jsx: true
                },
                transform: {
                  react: {
                    development: isDev,
                    refresh: false
                  }
                }
              }
            }
          }
        ]
      },
      {
        test: /\.(png|jpg|jpeg|gif|svg)$/,
        type: 'asset/resource'
      }
    ]
  },
  plugins: [
    new rspack.container.ModuleFederationPlugin({
      name: 'host',
      remotes: {
        remote: 'remote@http://localhost:3002/remoteEntry.js'
      },
      shared: {
        react: {
          singleton: true,
          requiredVersion: '^18.3.1',
          eager: true
        },
        'react-dom': {
          singleton: true,
          requiredVersion: '^18.3.1',
          eager: true
        },
        'react-router-dom': {
          singleton: true,
          requiredVersion: '^7.1.1',
          eager: true
        },
        antd: {
          singleton: true,
          requiredVersion: '^5.21.8',
          eager: true
        },
        '@ant-design/icons': {
          singleton: true,
          requiredVersion: '^5.5.2',
          eager: true
        },
        '@reduxjs/toolkit': {
          singleton: true,
          requiredVersion: '^2.5.0',
          eager: true
        },
        'react-redux': {
          singleton: true,
          requiredVersion: '^9.2.0',
          eager: true
        },
        'lodash-es': {
          singleton: true,
          requiredVersion: '^4.17.21',
          eager: true
        },
        'chart.js': {
          singleton: true,
          requiredVersion: '^4.4.7',
          eager: true
        },
        'react-chartjs-2': {
          singleton: true,
          requiredVersion: '^5.2.0',
          eager: true
        },
        dayjs: {
          singleton: true,
          requiredVersion: '^1.11.13',
          eager: true
        }
      }
    }),
    new rspack.HtmlRspackPlugin({
      template: './src/index.html'
    }),
    // isDev && new refreshPlugin() // Disabled
  ].filter(Boolean),
  output: {
    path: path.resolve(__dirname, 'dist'),
    publicPath: 'http://localhost:3001/',
    clean: true
  },
  devServer: {
    port: 3001,
    hot: false,
    historyApiFallback: true,
    headers: {
      'Access-Control-Allow-Origin': '*'
    }
  }
});