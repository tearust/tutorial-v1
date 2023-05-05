import Vue from 'vue'
import App from './App.vue'

import './elementui-style/index.css';
import ElementUI from 'element-ui';
import { Loading } from 'element-ui';
import locale from 'element-ui/lib/locale/lang/en';

import router from './router';
import './style.scss';

import store from './store';
import utils from './tea/utils';
import { _ } from 'tearust_utils';

import layer1_error_tips from './assets/error';
import strings from './assets/string';
import helper from './views/helper';

import './filter';
import layer2 from './layer2';
import {ContractMap} from './eth/consts';



Vue.use(ElementUI, { locale });
Vue.config.productionTip = false;

router.beforeEach((to, from, next) => {
  const layer1_ready = utils.mem.get('layer1_ready');

  if (layer1_ready && to.meta && to.meta.needLogin) {
    const user = store.state.user;
    if(!user){
      return router.replace({ path: '/login_page' }).catch(()=>{})
    }
  }

  next();
});

const C = {
  
};
new Vue({
  router,
  store,
  methods: {
    is_dev() {
      return utils.is_dev();
    },
    loading(f, text = 'Loading...') {
      if (f) {
        if(C._loading){
          C._loading.$el.querySelector('.el-loading-text').innerHTML = text;
        }
        else{
          C._loading = Loading.service({
            lock: true,
            text,
            customClass: 'c-fullscreen-loading',
            spinner: 'el-icon-loading',
            background: 'rgba(0, 0, 0, 0.05)'
          });
        }
        
      }
      else {
        C._loading && C._loading.close();
        C._loading = null;
      }
    },
    processError(e){
      if(_.includes(e.toString(), '"login" cannot be none')){
        this.$alert("Login session expired.", 'Session error', {
          type: 'error'
        });
        _.delay(()=>{
          layer2.user.logout();
        }, 2000);
      }
    },
    showError(e, title = 'Error message') {
      console.log(e);
      try{
        const json = JSON.parse(e.toString());
        
        return this.$alert(json.human || json.summary, title, {
          type: 'error',
          dangerouslyUseHTMLString: true,
        });
      }catch(ee){}
      
      if(_.includes(e.toString(), 'not_login')){
        _.delay(()=>{
          layer2.user.logout();
        }, 2000);
      }

      let err = e.message || e.toString();
      const [f, ss, OP, code, error_str] = utils.tappLayer2FormatError(err);
      let default_error;
      if(!f){
        err = ss;
        default_error = ss;
      }
      else{
        err = ss;
        default_error = error_str;
      }
      let ex = _.get(layer1_error_tips, err, default_error);
      this.$alert(ex, title, {
        type: 'error',
        dangerouslyUseHTMLString: true,
      });
    },
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
    str(key){
      return _.get(strings, key, key);
    },
    goPath(path, type="push"){
      this.$router[type](path).catch(()=>{});
    },
    openUrl(url){
      helper.openUrl(url);
    },
    is_sudo(address){
      return this.$root.eq(utils.get_env('LAYER1_SUDO'), address);
    },
    eq(str1, str2){
      return _.toLower(str1) === _.toLower(str2);
    },
    is_tappstore(address){
      return _.toLower(address) === _.toLower(ContractMap.ERC20);
    },
    go_wallet(url){
      helper.go_wallet(url);
    },

    
  },
  render: h => {
    return h(App);
  },
}).$mount('#app');
