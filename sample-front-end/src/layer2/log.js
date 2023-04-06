import {_, moment} from 'tearust_utils';
import utils from '../tea/utils';

import base from './base';
import txn from './txn';
import user from './user';

const F = {
  
  async queryOpLogs(self, param={}){

    self.$root.loading(true);
    
    const opts = {};
    opts.address = self.layer1_account.address;
    if(param.address){
      opts.target = param.address;
    }
    if(param.day){
      opts.day = param.day;

      opts.month = param.month || moment.utc().month()+1;
      opts.year = param.year || moment.utc().year();
    }

    try{
      const rs = await txn.query_request('queryOpLogs', opts, true);
      self.$root.loading(false);

      rs.logs = _.map(rs.logs, (item)=>{
        item.statement_type = _.get({
          'Incoming': 'Received',
          'Outcoming': 'Paid',
        }, item.statement_type, item.statement_type);

        item.state_type = _.get({
          'Bonding': 'Token on bonding curve',
        }, item.state_type, item.state_type);

        item.memo = _.get({
          'seat for bonding curve cronjob': 'Global token reward',
          'seat main cronjob': 'Maintainer seat reward',
        }, item.memo, item.memo||'Others');

        return item;
      });

      return _.reverse(rs.logs);
      
    }catch(e){
      self.$root.loading(false);
      console.log('queryOpLogs error:', e);
    }
    
  },

  
  
};

export default F;