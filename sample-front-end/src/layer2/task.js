import { _ } from 'tearust_utils';
import utils from '../tea/utils';
import store from '../store';
import { stringToHex, hexToU8a, stringToU8a, numberToHex, u8aToString } from '@polkadot/util';

import base from './base';
import txn from './txn';
import eth from '../eth';
import common from './common';
import user from './user';

const reg_tweet = /^https:\/\/twitter.com\/(.+)\/status\/(\d+)/i;

const F = {
  async initDB(self, succ_cb){
    self.$root.loading(true);
    try {
      await txn.txn_request('init_db', {
        tokenId: base.getTappId(),
        address: self.layer1_account.address,
      });
      self.$root.alert_success();
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
      
    }
    self.$root.loading(false);
  },
  async initToken(self, succ_cb){
    self.$root.loading(true);
    try {
      await txn.txn_request('init_token', {
        tokenId: base.getTappId(),
        address: self.layer1_account.address,
      });
      self.$root.alert_success();
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);
  },
  async createNewTask(self, param={}, succ_cb){
    const session_key = user.checkLogin(self);

    const tappId = base.getTappId();
    self.$store.commit('modal/open', {
      key: 'common_form',
      param: {
        title: 'Create Task',
        text: ``,
        props: {
          subject: {
            type: 'Input',
            required: true,
            label: 'Tweet URL',
            tip: "The tweet url you want retweeted."
          },
          price: {
            type: 'number',
            default: 10,
            label: 'Price (TEA)'
          },
          required_deposit: {
            type: 'number',
            default: 20,
            label: 'Deposit (TEA)',
          }

        },
      },
      cb: async (form, close) => {
        self.$root.loading(true);
        const price = utils.layer1.amountToBalance(form.price);
        const required_deposit = utils.layer1.amountToBalance(form.required_deposit);

        let tweet = form.subject;
        if(reg_tweet.test(tweet)){
          const arr = tweet.match(reg_tweet);
          tweet = arr[2];
        }
        const param = {
          address: self.layer1_account.address,
          tappIdB64: tappId,
          authB64: session_key,
          price: utils.toBN(price).toString(),
          requiredDeposit: utils.toBN(required_deposit).toString(),
          subject: tweet,
        };

        try {
          await txn.txn_request('create_task', param);
          await succ_cb();
        } catch (e) {
          console.error(e);
          self.$root.showError(e.toString());
        }
        close();
        self.$root.loading(false);

      }
    });
  },
  async deleteTask(self, param, succ_cb){
    const session_key = user.checkLogin(self);
    try{
      await self.$confirm('Are you sure to delete the task?', {
        title: 'Delete task',
        dangerouslyUseHTMLString: true,
      });
    }catch(e){
      return;
    }

    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
      subject: param.subject,
    };

    try {
      await txn.txn_request('delete_task', opts);
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);
  },
  async takeTask(self, param, succ_cb){
    const session_key = user.checkLogin(self);
    try{
      await self.$confirm('Are you sure to take the task?', {
        title: 'Take task',
        dangerouslyUseHTMLString: true,
      });
    }catch(e){
      return;
    }

    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
      subject: param.subject,
    };

    try {
      await txn.txn_request('take_task', opts);
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);
  },
  async completeTask(self, param, succ_cb, error_cb){
    const session_key = user.checkLogin(self);

    self.$store.commit('modal/open', {
      key: 'common_form',
      param: {
        title: 'Complete Task',
        text: `Quote retweet the following tweet and enter your quote retweet url below to complete the task.<br/>
        Please quote retweet <a target="_blank" href="https://twitter.com/u/status/${param.subject}">https://twitter.com/u/status/${param.subject}<a/><br/><img style="width:160px;margin-top:10px; margin-left:150px;" src="/quote.png" />`,
        props: {
          text: {
            type: 'Input',
            required: true,
            label: 'Quote Retweet URL',
          },

        },
      },
      cb: async (form, close) => {
        self.$root.loading(true);

        let tweet = form.text;
        if(reg_tweet.test(tweet)){
          const arr = tweet.match(reg_tweet);
          tweet = arr[2];
        }

        const opts = {
          address: self.layer1_account.address,
          tappIdB64: base.getTappId(),
          authB64: session_key,
          subject: param.subject,
          text: tweet
        };

        try {
          await txn.txn_request('complete_task', opts);
          await succ_cb();
        } catch (e) {
          console.error(e);
          self.$root.showError(e.toString());
          if(error_cb){
            await error_cb();
          }
        }
        close();
        self.$root.loading(false);

      }
    });
  },
  async verifyTask(self, param, succ_cb){
    const session_key = user.checkLogin(self);
    try{
      await self.$confirm('Are you sure to '+(param.ok?'Confirm':'Reject')+' the task?', {
        title: 'Verify task',
        dangerouslyUseHTMLString: true,
      });
    }catch(e){
      return;
    }

    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
      subject: param.subject,
      failed: param.ok ? false : true,
    };

    try {
      await txn.txn_request('verify_task', opts);
      self.$root.success();
      await succ_cb();
    } catch (e) {
      console.error(e);
      self.$root.showError(e.toString());
    }
    self.$root.loading(false);
  },
  async queryTaskList(self){
    const rs = await txn.query_request('query_task_list', {
      address: self.layer1_account.address,
    });
    return _.map(rs.list, (item)=>{
      try{
        item.price = utils.layer1.balanceToAmount(item.price);
        item.deposit = utils.layer1.balanceToAmount(item.required_deposit);
      }catch(e){
        item.price = utils.layer1.balanceToAmount(utils.toBN('0x'+item.price).toString());
        item.deposit = utils.layer1.balanceToAmount(utils.toBN('0x'+item.required_deposit).toString());
      }
      
      return item;
    });
  }


};

export default F;