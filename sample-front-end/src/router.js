import Vue from 'vue'
import Router from 'vue-router'

import Home from './views/Home';
import LoginPage from './views/LoginPage';
// import Welcome from './views/Welcome';
import AccountProfile from './views/AccountProfile';

Vue.use(Router);


let routers = [
  {
    path: '/',
    redirect: '/home',
  },
  {
    path: '/home',
    name: 'home',
    component: Home,
  },
  {
    path: '/login_page',
    name: 'login_page',
    component: LoginPage,
  },
  {
    path: '/account_profile',
    name: 'account_profile',
    component: AccountProfile,
    meta: {
      needLogin: true,
    }
  },
  
  
  
];

export default new Router({
  routes: routers
})