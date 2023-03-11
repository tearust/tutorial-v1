import Vue from 'vue'
import App from './App.vue'

import './elementui-style/index.css';
import ElementUI from 'element-ui';
// import { Loading } from 'element-ui';
import locale from 'element-ui/lib/locale/lang/en';

import router from './router';
import { _ } from 'tearust_utils';


Vue.use(ElementUI, { locale });
Vue.config.productionTip = false;

router.beforeEach((to, from, next) => {
  next();
});

new Vue({
  router,
  methods: {
    alert_success(message='Success', title='', cb=null){
      this.$alert(message, title, {
        type: 'success',
        dangerouslyUseHTMLString: true,
        callback: cb || (()=>{}),
      });
    },
    success(message='', type='success'){
      const title = {
        'success': 'Success',
        'error': 'Error',
      }[type];
      this.$notify({
        title: title || '',
        message,
        customClass: 'tea-notify',
        type,
        duration: 2500
      });
    },
    goPath(path, type="push"){
      this.$router[type](path).catch(()=>{});
    },
  },
  render: h => {
    return h(App);
  },
}).$mount('#app');
