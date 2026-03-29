module.exports = {
    publicPath: '/',
    devServer: {
      proxy: {
        '/api': {
          target: 'http://127.0.0.1:5540',
          changeOrigin: true
        }
      }
    }
  }

  //TODO: test db
  //TODO: update .htpasswd based on this