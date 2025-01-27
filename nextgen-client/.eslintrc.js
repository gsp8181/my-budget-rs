module.exports = {
    extends: [
      'plugin:import/recommended',
      'plugin:import/errors',
      'plugin:import/warnings'
    ],
    plugins: [
      'import'
    ],
    rules: {
      'import/prefer-default-export': 'off'
    },
    settings: {
      'import/resolver': {
        node: {
          extensions: ['.js', '.jsx']
        }
      }
    }
  };