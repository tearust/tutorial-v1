import {_, axios, moment, uuid} from 'tearust_utils';
import utils from '../tea/utils';
import base from './base';
import { NOT_LOGIN } from './user';
import mem from './mem';
import user from './user';

const F = {

  async txn_request(method, param){
    const _uuid = uuid();
    console.log("prepare for txn: ", method, _uuid);
    
    const _axios = base.getAxios();

    const txn_uuid = 'txn_'+_uuid;
    try{
      base.log("Send txn request...");
      console.log("Send txn request...");
      const step1_rs = await _axios.post('/'+method, {
        ...param,
        uuid: txn_uuid,
      });
      console.log("step_1 result: ", step1_rs);
    }catch(e){
      console.error("step_1 error: ", e);

      throw e;
    }

    mem.clear();

    base.log('Wait for query txn hash...');
    console.log('Wait for query txn hash...');
    await utils.sleep(5000);

    let step_2_rs = null;
    const step_2_loop = async ()=>{
      try{
        console.log('query result for '+txn_uuid+'...');
        step_2_rs = await _axios.post('/query_result', {
          uuid: txn_uuid,
        });

      }catch(e){
        console.log("step2 error: ", e);
        // rs = e.message;
        step_2_rs = null;
        await utils.sleep(3000);
        await step_2_loop();
      }
  
    };
  
    base.log("Querying txn result ...");
    console.log("Querying txn result ...");
    await step_2_loop();

    console.log("step2 result: ", step_2_rs);

    base.log('Wait for next step...');
    console.log('Wait for next step...');
    utils.sleep(5000);

    const step_3_hash = step_2_rs.hash;
    const step_3_ts = step_2_rs.ts;
    const hash_uuid = "hash_"+_uuid;
    let step_3_rs = null;
    let step_4_rs = null;
    let sn = 0;
    const step_4_loop = async ()=>{
      if(sn > 1) {  // TODO
        step_4_rs = {
          'status': false,
          'error': 'request timeout',
        };
        return;
      }
      try{
        base.log("Send query txn hash request...");
        console.log('Send query txn hash request...');
        step_3_rs = await _axios.post('/queryHashResult', {
          hash: step_3_hash,
          ts: step_3_ts.toString(),
          uuid: hash_uuid,
        });
    
        base.log('Wait for query txn hash result...');
        console.log('Wait for query txn hash result...');
        await utils.sleep(5000);

        console.log('query hash result for '+hash_uuid+'...');
        step_4_rs = await _axios.post('/query_result', {
          uuid: hash_uuid,
        });

        if(!step_4_rs.status) throw step_4_rs.error;
      }catch(e){
        console.log("step4 error: ", e);

        if(e !== 'wait'){
          throw e;
        }
        
        // rs = e.message;
        step_4_rs = null;
        sn++;
        await utils.sleep(5000);
        await step_4_loop();
      }
  
    };
  
    base.log("Start to query hash result...");
    console.log("Start to query hash result...");
    await step_4_loop();

    console.log("step4 result: ", step_4_rs);
    if(step_4_rs.error){
      throw step_4_rs.error;
    }
    else{
      // if param include auth_key, means need to login, will extend the session_key
      if(_.has(param, 'authB64') && _.has(param, 'address')){
        console.log('extend session key');
        user.extendSession(param.address, param.authB64);
      }
    }

    if(!step_4_rs.need_query){
      return step_4_rs;
    }

    // continue query

    let step_5_rs = null;
    let step_5_uuid = step_4_rs.query_uuid || _uuid;
    let step_5_n = 0;
    const step_5_loop = async ()=>{
      if(step_5_n > 3){
        throw 'query timeout...';
      }
      try{
        console.log('continue query for '+step_5_uuid+'...');
        step_5_rs = await _axios.post('/query_result', {
          uuid: step_5_uuid,
        });

      }catch(e){
        console.log("step5 error: ", e);
        step_5_n ++;
        step_5_rs = null;
        await utils.sleep(5000);
        await step_5_loop();
      }
    };

    base.log("Querying action result ...");
    console.log("Querying action result ...");
    await step_5_loop();
    console.log("step5 result: ", step_5_rs);

    const rs = step_5_rs;

    
    return rs;
  },

  async query_request(method, param){
    const _uuid = uuid();
  
    // base.log("Start to first query request...");
    console.log("Start to first query request...");
    const _axios = base.getAxios();

    try{
      const step1_rs = await _axios.post('/'+method, {
        ...param,
        uuid: _uuid,
      });
      console.log('first step result => '+step1_rs);
    }catch(e){
      throw e;
  
      // base.log("Continue query...");
      console.log("Continue query...");
    }
    
    await utils.sleep(5000);

    // base.log("Start to second query request...");
    console.log("Start to second query request...");
  
    let rs = null;
    let n = 0;
    const loop2 = async ()=>{
      if(n>10){
        throw "timeout, please retry.";
      }
      try{
        rs = await _axios.post('/query_result', {
          uuid: _uuid,
        });
  
      }catch(e){
        // rs = e.message;
        rs = null;
        await utils.sleep(5000);
        n++;
        
        await loop2();
      }
  
    };
  
    await loop2();
  
    if(rs){
      console.log('query_request result: ', rs);
      return rs;
    }
  
    return rs;
  }

};


export default F;