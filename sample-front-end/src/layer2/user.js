import { _ } from 'tearust_utils';
import utils from '../tea/utils';
import store from '../store';
import { stringToHex, hexToU8a, stringToU8a, numberToHex, u8aToString } from '@polkadot/util';

import base from './base';
import txn from './txn';
import eth from '../eth';
import common from './common';

export const NOT_LOGIN = 'not_login';

const F = {
  getUserId(address) {
    return `profile__${address}`;
  },
  getOfflineId(){
    return 'tea_offline_id';
  },
  current(address) {
    const key = F.getUserId(address);
    const user = utils.cache.get(key);
    if (user && user.expird_time && user.expird_time > Date.now()) {
      return user;
    }
    utils.cache.remove(key);
    return null;
  },
  extendSession(address, session_key) {
    const user = {
      address,
      isLogin: true,
      session_key: session_key,
      expird_time: Date.now() + 1800 * 1000,
    };

    utils.cache.put(F.getUserId(address), user);

  },
  checkLogin(self) {
    const address = self.layer1_account.address;
    // const _axios = base.getAxios();
    const user = F.current(address);
    if (!user || !user.isLogin) {

      self.$root.alert_success('Session expired or Not login', 'Please login again.', ()=>{
        F.logout();
      });
      
      return 'NA';
    }

    return user.session_key;
  },

  async login(self, permission_str) {
    const address = self.layer1_account.address;

    const chain = await self.wf.layer1.getChain();
    if(chain.name === 'Offline'){
      throw('You did not install metamask wallet, please login with your email address.');
    }


    // thanks for https://github.com/polkadot-js/extension/issues/827
    const data = permission_str;
    console.log('permission_str => ' + permission_str);

    try {

      const layer1_instance = self.wf.getLayer1Instance();
      let [sig, pk, msg_bytes, msg] = await layer1_instance.signMessage(data);


      sig = sig.replace(/^0x/, '');
      let rs = await txn.txn_request('login', {
        tappIdB64: base.getTappId(),
        address,
        pk: utils.uint8array_to_base64(hexToU8a(pk)),
        data: msg,
        signature: sig,
      });
      rs = await txn.query_request('query_session_key', {
        tappIdB64: base.getTappId(),
        address,
      });

      if (rs.auth_key) {
        const user = {
          address,
          isLogin: true,
          session_key: rs.auth_key,
          expird_time: Date.now() + 1800 * 1000,
        };

        utils.cache.put(F.getUserId(address), user);
        await store.dispatch('init_user');

        base.top_log(null);

        self.$root.goPath('/account_profile');
        return true;
      }

    } catch (e) {
      // TODO handle error.
      throw e;
    }
  },


  async logout(address = null) {
    const _axios = base.getAxios();
    address = address || store.getters.layer1_account.address;
    if (address) {
      utils.cache.remove(F.getUserId(address));
      utils.cache.remove(F.getOfflineId());
    }
    await utils.sleep(500);
    location.reload(true);
  },

  async showLoginModal(self, succ_cb = null) {
    if (!self.layer1_account || !self.layer1_account.address) {
      self.$root.showError("Invalid user, please select.");
      return;
    }

    self.$store.commit('modal/open', {
      key: 'login',
      param: {},
      cb: async (permission_str, close) => {

        self.$root.loading(true);
        try {
          await F.login(self, permission_str);

          self.$root.success('Login success.');
          if (succ_cb) {
            await succ_cb();
          }
        } catch (e) {
          self.$root.showError(e.reason || e.toString());
        }

        close();
        self.$root.loading(false);

      }
    })

  },

  async topupFromLayer1(self, succ_cb) {
    const layer1_instance = self.wf.getLayer1Instance();

    self.$store.commit('modal/open', {
      key: 'common_form',
      param: {
        title: 'Topup',
        text: 'Move chain wallet (layer1) TEA funds to layer2 TApp Store wallet account',
        props: {

          amount: {
            type: "number",
            default: 10,
            max: 9999999999,
            label: "Amount (TEA)"
          }
        },
      },
      cb: async (form, close) => {

        const amt = _.toNumber(form.amount);
        if(self.layer1_account.balance < amt){
          self.$root.showError("Not enough balance to topup.");
          return false;
        }
        if(amt < 1) {
          self.$root.showError('Minimum topup amount is 1T.');
          return false;
        }

        self.$root.loading(true);
        try {
          await layer1_instance.topup(amt);
        } catch (e) {
          self.$root.showError(e);
          close();
          self.$root.loading(false);
          return;
        }

        close();

        await succ_cb()
        self.$root.loading(false);
      },

    });
  },

  async withdrawFromLayer2(self, amt, succ_cb) {
    const session_key = F.checkLogin(self);

    const tappId = base.getTappId();
    self.$store.commit('modal/open', {
      key: 'common_form',
      param: {
        title: 'Withdraw',
        text: `Move TEA funds back to chain wallet (layer1)<br/>${utils.consts.gas_tip()}`,
        props: {
          amount: {
            type: 'number',
            default: amt,
            label: 'Amount (TEA)'
          }
        },
      },
      cb: async (form, close) => {
        self.$root.loading(true);
        const amount = utils.layer1.amountToBalance(form.amount);

        const param = {
          address: self.layer1_account.address,
          tappIdB64: tappId,
          authB64: session_key,
          amount: utils.toBN(amount).toString(),
        };

        try {
          await txn.txn_request('withdraw', param);
          self.$root.success();
          succ_cb();
        } catch (e) {

          self.$root.showError(e);
        }
        close();
        self.$root.loading(false);

      }
    });
  },

  

  async transferTea(self, param = {}, succ_cb) {
    const session_key = F.checkLogin(self);

    const tappId = base.getTappId();
    self.$store.commit('modal/open', {
      key: 'common_form',
      param: {
        title: 'Transfer TEA',
        label_width: 200,
        text: `Transfer TEA to another user using your TApp store wallet (layer2). ${utils.consts.gas_tip()}`,
        props: {
          target: {
            type: 'Input',
            label: 'Target address or email',
            required: true,
          },
          amount: {
            type: 'number',
            default: 1,
            label: 'Amount',
            min: 1,
          }
        },
      },
      cb: async (form, close) => {
        let send_email = false;
        
        const amount = utils.layer1.amountToBalance(form.amount);
        let tar = form.target;
        if(!utils.isEmail(tar) && !eth.help.getUtils().isAddress(tar)){
          self.$root.showError('Invalid address or email');
          return false;
        }

        if(utils.isEmail(tar)){
          tar = utils.emailToAddress(tar);
          send_email = {
            from_address: self.layer1_account.address,
            email: form.target,
            amount: form.amount
          };
        }

        const param = {
          address: self.layer1_account.address,
          tappIdB64: tappId,
          authB64: session_key,
          amount: utils.toBN(amount).toString(),
          to: tar,
        };

        self.$root.loading(true);
        try {
          await txn.txn_request('transferTea', param);

          if(send_email){
            await F.send_email_for_transfer_tea(send_email.from_address, send_email.email, send_email.amount);
          } 
          self.$root.success();
          succ_cb();
        } catch (e) {

          self.$root.showError(e);
        }
        close();
        self.$root.loading(false);

      }
    });
  },

  async query_balance(self, target = null, target_tapp = null,) {
    const session_key = F.checkLogin(self);

    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
    };
    if (target) {
      opts.target = target;
      opts.targetTappIdB64 = target_tapp;
    }

    try {
      const rs = await txn.query_request('query_balance', opts);
      if (!rs.balance) {
        rs.balance = 0;
      }

      return rs ? utils.layer1.balanceToAmount(rs.balance) : null;
    } catch (e) {
      self.$root.showError(e);

      return 0;
    }

  },
  async query_deposit(self) {
    const session_key = F.checkLogin(self);
    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
    };

    const rs = await txn.query_request('query_deposit', opts);
    if (!rs.balance) {
      rs.balance = 0;
    }

    return rs ? utils.layer1.balanceToAmount(rs.balance) : null;
  },

  async query_asset(self, target = null) {
    const session_key = F.checkLogin(self);

    const opts = {
      address: self.layer1_account.address,
      tappIdB64: base.getTappId(),
      authB64: session_key,
    };
    if (target) {
      opts.target = target;
    }

    const rs = await txn.query_request('query_asset', opts);

    const json = {
      tea_balance: utils.layer1.balanceToAmount(rs.tea_balance),
      token_balance: utils.layer1.balanceToAmount(rs.token_balance),
      reserved_token_balance: utils.layer1.balanceToAmount(rs.reserved_token_balance),
    };
    console.log('user asset => ', json);
    return json;
  },





};

export default F;