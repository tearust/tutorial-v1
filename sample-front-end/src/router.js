import Vue from 'vue'
import Router from 'vue-router'

import Home from './views/Home';

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
  
  
  
];

export default new Router({
  routes: routers
})