import axios from 'axios';
import _ from 'lodash';
import utils from './tea/utils';

const LAYER1_RPC = utils.get_env('LAYER1_RPC');

const F = {


  async layer1_rpc(method, params=[]){
    const data = {
      jsonrpc: '2.0',
      method,
      params,
      id: 9999
    };
    const rs = await axios.post(LAYER1_RPC, data, {
      headers: {
        'Content-Type': 'application/json',
      }
    });

    if(rs.data.error){
      throw rs.data.error;
    }

    if(rs.data.id === 9999){
      return rs.data.result;
    }

    return null;
  }
};


export default F;