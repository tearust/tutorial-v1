import {_} from 'tearust_utils';
import utils from '../tea/utils';

import base from './base';
import txn from './txn';
import user from './user';
import mem from './mem';

const F = {
  async setAllowance(self, data, succ_cb){
    const session_key = user.checkLogin(self);


    let current_balance = 0;

    self.$store.commit('modal/open', {
      key: 'common_form', 
      param: {
        title: 'Set spending limit',
        confirm_text: 'Confirm',
        text: ``,
        props: {
          tapp_id: {
            label: 'Name',
            type: 'Input',
            disabled: true,
            default: data.name,
          },
          amount: {
            label: 'Amount',
            type: 'number',
            max: 100000000,
            default: 100,
          },
        },
      },
      cb: async (form, close)=>{
        if(!form.amount){
          form.amount = 0;
        }

        if(current_balance < 1){
          self.$root.showError("You'll need to Topup funds to your TApp Store wallet account before you can set spending limits.");
          return false;
        }

        const amount = utils.layer1.amountToBalance(form.amount);

        self.$root.loading(true);
        try{
          const opts = {
            tappIdB64: base.getTappId(),
            address: self.layer1_account.address,

            targetTappIdB64: base.getTappId(),
            amount: utils.toBN(amount).toString(),
            authB64: session_key,
          };

          const rs = await txn.txn_request('setAllowance', opts);
          console.log('setAllowance result:', rs);

          close();
          self.$root.success();
          await succ_cb();
        }catch(e){
          self.$root.showError(e);
        }
        self.$root.loading(false);
      },
      async open_cb(param){
        const r = await user.query_balance(self);
        current_balance = r;
      }
    });
  },
};
export default F;