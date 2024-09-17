//import 'bootstrap/dist/css/bootstrap.min.css';
import 'startbootstrap-sb-admin-2/css/sb-admin-2.min.css'
import $ from 'jquery'

window.jQuery = $;
window.$ = $;

import 'jquery.easing'
import 'bootstrap';

//chart.js/dist/Chart.min.js

import Vue from 'vue'
import axios from 'axios'
import App from './App.vue'
//import 'startbootstrap-sb-admin-2/js/sb-admin-2.min.js' TODO: fix
import router from './router'


Vue.config.productionTip = true

new Vue({
  router,
  render: h => h(App)
}).$mount('#app')
