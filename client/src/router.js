import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'

Vue.use(Router)

export default new Router({
  //mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    //{
    //  path: '/about',
    //  name: 'about',
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
    //  component: () => import(/* webpackChunkName: "about" */ './views/test.vue')
//},
    //{
    //  path: '/test',
    //  name: 'Test',
    //  component: () => import(/* webpackChunkName: "about" */ './views/test.vue')
    //},
    {
      path: '/bank',
      name: 'Bank',
      component: () => import(/* webpackChunkName: "bank" */ './views/bank.vue')
    },
    {
      path: '/cardbalance',
      name: 'CardBalance',
      component: () => import(/* webpackChunkName: "cardbalance" */ './views/card-balance.vue')
    },
    {
      path: '/cardheld',
      name: 'CardHeld',
      component: () => import(/* webpackChunkName: "cardheld" */ './views/card-held.vue')
    },
    {
      path: '/cash',
      name: 'Cash',
      component: () => import(/* webpackChunkName: "cash" */ './views/cash.vue')
    },
    {
      path: '/debt',
      name: 'Debt',
      component: () => import(/* webpackChunkName: "debt" */ './views/debt.vue')
    },
    {
      path: '/misccredit',
      name: 'MiscCredit',
      component: () => import(/* webpackChunkName: "misccredit" */ './views/misccredit.vue')
    },
    {
      path: '/miscdebit',
      name: 'MiscDebit',
      component: () => import(/* webpackChunkName: "miscdebit" */ './views/miscdebit.vue')
    },
    {
      path: '/debtto',
      name: 'OwedTo',
      component: () => import(/* webpackChunkName: "owedto" */ './views/owedto.vue')
    },
    {
      path: '/regularcredit',
      name: 'RegularCredit',
      component: () => import(/* webpackChunkName: "regularcredit" */ './views/regular-credit.vue')
    },
    {
      path: '/regularpayment',
      name: 'RegularPayment',
      component: () => import(/* webpackChunkName: "regularpayment" */ './views/regular-payment.vue')
    },
    {
      path: '/uncleared',
      name: 'Uncleared',
      component: () => import(/* webpackChunkName: "uncleared" */ './views/uncleared.vue')
    },
    {
      path: '/settings',
      name: 'Settings',
      component: () => import(/* webpackChunkName: "settings" */ './views/settings.vue')
    }
  ]
})

