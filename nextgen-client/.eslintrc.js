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
      'import/prefer-default-export': 'off',
      // MUI v6 sub-path exports and some peer deps aren't resolvable by the
      // static ESLint import resolver; the webpack bundler handles them fine.
      'import/no-unresolved': ['error', { ignore: ['^@mui/', '^@react-spring/'] }]
    },
    settings: {
      'import/resolver': {
        node: {
          extensions: ['.js', '.jsx']
        }
      }
    }
  };