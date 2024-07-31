const path = require('path');

module.exports = {
  entry: {
    background: './scripts/background.js',
    popup: './scripts/popup.js',
    content: './scripts/content.js'
  },
  output: {
    filename: '[name].bundle.js',
    path: path.resolve(__dirname, 'dist')
  },
  mode: 'production'
};
