import { defineConfig } from '@rspack/cli';
import { rspack } from '@rspack/core';
import refreshPlugin from '@rspack/plugin-react-refresh';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const isDev = process.env.NODE_ENV === 'development';

export default defineConfig({
  context: __dirname,
  entry: './src/index.jsx',
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
                    refresh: isDev
                  }
                }
              },
              rspackExperiments: {
                import: [{
                  libraryName: 'antd',
                  style: false
                }]
              }
            }
          }
        ]
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader']
      },
      {
        test: /\.less$/,
        use: [
          'style-loader',
          'css-loader',
          {
            loader: 'less-loader',
            options: {
              lessOptions: {
                javascriptEnabled: true
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
      name: 'remote',
      filename: 'remoteEntry.js',
      exposes: {
        './UserCard': './src/components/UserCard',
        './DataTable': './src/components/DataTable',
        './ChartWidget': './src/components/ChartWidget',
        './FormBuilder': './src/components/FormBuilder',
        './store': './src/store/slices',
      },
      shared: {
        react: {
          singleton: true,
          requiredVersion: '^18.3.1'
        },
        'react-dom': {
          singleton: true,
          requiredVersion: '^18.3.1'
        },
        antd: {
          singleton: true,
          requiredVersion: '^5.21.8'
        },
        '@ant-design/icons': {
          singleton: true,
          requiredVersion: '^5.5.2'
        },
        '@reduxjs/toolkit': {
          singleton: true,
          requiredVersion: '^2.5.0'
        },
        'react-redux': {
          singleton: true,
          requiredVersion: '^9.2.0'
        },
        'lodash-es': {
          singleton: true,
          requiredVersion: '^4.17.21'
        },
        'chart.js': {
          singleton: true,
          requiredVersion: '^4.4.7'
        },
        'react-chartjs-2': {
          singleton: true,
          requiredVersion: '^5.2.0'
        },
        dayjs: {
          singleton: true,
          requiredVersion: '^1.11.13'
        }
      }
    }),
    new rspack.HtmlRspackPlugin({
      template: './src/index.html'
    }),
    isDev && refreshPlugin()
  ].filter(Boolean),
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].[contenthash].js',
    chunkFilename: '[name].[contenthash].js',
    publicPath: 'http://localhost:3002/',
    clean: true
  },
  devServer: {
    port: 3002,
    hot: true,
    headers: {
      'Access-Control-Allow-Origin': '*'
    }
  },
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          priority: 10
        }
      }
    }
  }
});