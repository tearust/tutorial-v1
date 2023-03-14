import {_} from 'tearust_utils';
import utils from '../tea/utils';

class REQ_MEM {
  constructor(){
    this.data = {};
    this.blocking = {};
    this.expire_time = 1000*60*1;
  }
  set_doing(key){
    this.blocking[key] = true;
  }
  set(key, val){
    const dd = [_.cloneDeep(val), Date.now()];
    this.data[key] = dd;
    this.blocking[key] = false;
  }
  get(key){
    const xd = this.data[key];
    if(!xd) return null;

    if(Date.now() - this.expire_time > xd[1]){
      return null;
    }
    return _.cloneDeep(xd[0]);
  }
  clear(){
    this.data = {};
    this.blocking = {};
  }
}
const request_mem = new REQ_MEM();
window.request_mem = request_mem;

utils.register('clear-mem-cache', ()=>{
  request_mem.clear();
});

export default request_mem;