import {_, axios, moment, uuid} from 'tearust_utils';
import utils from '../tea/utils';
import store from '../store';
import {ContractMap} from '../eth/consts';

const meta = {
  type: utils.get_env('TAPP_TYPE') || 'tapp',  // tappstore, miner, tapp
  sudo: utils.get_env('LAYER1_SUDO') || 'NO_SUDO_ACCOUNT',
  tapp_id: _.toLower(ContractMap.ERC20),
  server_url: utils.get_env('LAYER2_URL'),
  mode: utils.get_env('mode') || 'app',
  system_actor: 'com.tea.client-actor',
  server_actor: 'someone.sample',
  
};

const _axios = axios.create({
  baseURL: meta.server_url,
});

// set request header 
_axios.interceptors.request.use((config)=>{
  // if(config.data.sample){
    config.data.actor = meta.server_actor;
  //   delete config.data.sample;
  // }
  // else {
  //   config.data.actor = meta.system_actor;
  // }
  
  return config;
});

// set request response
_axios.interceptors.response.use((res)=>{
  if(res.data){
    if(res.data.error){
      return Promise.reject(res.data.error);
    }
    if(res.data.data){
      return Promise.resolve(res.data.data);
    }
    else{
      return Promise.resolve(res.data);
    }
  }
}, (error)=>{
  if(error.response && error.response.status === 503){
    const err = error.response.data.error.replace('Invocation failure: Failed to invoke guest call: Guest call failure: Guest call failed: ', '');
    return Promise.reject(err);
  }
  return Promise.reject(error);
});

let _log = console.log;
const F = {
  setLog(log_fn){
    _log = log_fn;
  },
  log(msg){
    _log(msg);

  },
  set_global_log(self){
    F.setLog((msg)=>{
      self.$root.loading(true, msg);
    });
  },
  set_special_log(self){
    F.setLog((msg)=>{
      // TODO
    });
  },
  top_log(html, level='success'){
    utils.publish('top_log', {
      top_log: html,
      top_log_level: level,
    });
  },
  getTappId(){
    return meta.tapp_id;
  },
  getAxios(){
    return _axios;
  },
};

export default F;