import Vue from 'vue'
import Router from 'vue-router'

import Home from './views/Home';
import LoginPage from './views/LoginPage';
import AccountProfile from './views/AccountProfile';
import TaskMain from './views/TaskMain';
import Admin from './views/Admin';

Vue.use(Router);


let routers = [
  {
    path: '/',
    redirect: '/welcome',
  },
  {
    path: '/welcome',
    name: 'welcome',
    component: Home,
  },
  {
    path: '/admin',
    name: 'admin',
    component: Admin,
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
  {
    path: '/task_main',
    name: 'task_main',
    component: TaskMain,
    meta: {
      needLogin: true,
    }
  },
  
  
  
];

export default new Router({
  routes: routers
})