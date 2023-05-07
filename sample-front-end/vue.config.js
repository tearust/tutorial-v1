module.exports = {
  lintOnSave: false,
  publicPath : (
    process.env.NODE_ENV === 'production'
  ) ? './' : '/',

  configureWebpack: {
    module: {
      rules: [
        {
          test: /\.js$/,
          loader: require.resolve('@open-wc/webpack-import-meta-loader'),
          exclude: /\.vue$/,
        },

        {
          test: /\.m?js$/,
          include: /node_modules[/\\|]@polkadot/i,
          // exclude: /(node_modules|bower_components)/,
          use: {
            loader: 'babel-loader',
            options: {
              presets: [
                '@babel/preset-env',
                '@vue/cli-plugin-babel/preset',
              ],
              plugins: [
                "@babel/plugin-proposal-private-methods",
                "@babel/plugin-proposal-class-properties",
                '@babel/plugin-proposal-object-rest-spread',
              ]
            }
          }
        },


      ]
    }
  },

  outputDir : 'dist',
  css: {
    extract: true,
    requireModuleExtension: true,
    loaderOptions: {
      sass: {
        prependData: `
          
        `
      }
    }
  },
  devServer: {
    port : 3202,
    https : false,
    disableHostCheck: true,
  }
}