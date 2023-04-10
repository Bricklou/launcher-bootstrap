const path = require('path')

const config = (config, options, targetOptions) => {
  /*  module: {
    rules: [
      {
        test: /\.css$/,
        loader: 'postcss-loader',
        options: {
          postcssOptions: {
            config: path.resolve(__dirname, './postcss.config.js')
          }
        }
      }
    ]
  }*/

  const out = config.module.rules.filter(
    (item) =>
      item.test.test('style.css') ||
      item.test.test('style.scss') ||
      item.test.test('style.sass') ||
      item.test.test('style.less') ||
      item.test.test('style.styl')
  )

  out.forEach(function (_, index, arr) {
    arr[index].rules.forEach(function (_, index2, arr2) {
      const useList = []
      if (arr2[index2].oneOf) {
        arr2[index2].oneOf.forEach(function (_, index3, arr3) {
          useList.push(...arr3[index3].use)
        })
      }

      if (arr2[index2].use) {
        useList.push(...arr2[index2].use)
      }

      useList.forEach(function (_, index3, arr3) {
        if (arr3[index3].loader.includes('postcss')) {
          if (arr3[index3].options?.postcssOptions) {
            arr3[index3].options.postcssOptions.config = path.resolve(
              __dirname,
              './postcss.config.js'
            )
          }
        }
      })
    })
  })

  config.resolve.alias = {
    '@': path.resolve(__dirname, './src'),
    '@app': path.resolve(__dirname, './src/app')
  }

  return config
}

module.exports = config