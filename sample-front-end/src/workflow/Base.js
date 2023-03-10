import layer2 from '../layer2';
import eth from '../eth';
import utils from '../tea/utils';
import Log from '../shared/utility/Log';
import http from '../tea/http';
import store from '../store';
import request from '../request';

import { _, forge, moment } from 'tearust_utils';
import { hexToString, numberToHex } from '@polkadot/util';

import '../tea/moment-precise-range';


let _layer1 = null;
let _init = false;
export default class {
  constructor() {
    this.layer1 = _layer1;
    this._log = Log.create(this.defineLog());

  }

  defineLog() {
    return 'Base';
  }

  async __init__(){
    await this.initLayer1();
    _init = true;
  }

  async init() {
    if(_init){
      return true;
    }

    const init_loop = async (resolve) => {
      const layer1_ready = utils.mem.get('layer1_ready');
      if (!_init || !layer1_ready) {
        _.delay(() => {
          init_loop(resolve);
        }, 300);
      }
      else {
        resolve();
      }
    };

    return new Promise(async (resolve) => {
      if (!_init) {
        await this.initLayer1();
        _init = true;
      }
      await init_loop(resolve);
    });

  }

  async getAllLayer1Account() {
    const layer1_instance = this.getLayer1Instance();
    if (layer1_instance && layer1_instance.extension) {
      const all_account = await layer1_instance.extension.getAllAccounts();

      return all_account;
    }

    return [];
  }

  async initLayer1() {
    if (!_layer1) {
      _layer1 = await eth.get();

      this.layer1 = _layer1;
      await this.initEvent();
    }
  }

  async initEvent() {
    utils.register('layer1_block', (key, {block})=>{
      store.commit('set_chain', {
        current_block: block,
      });
      if(utils.getEpochEndBlock() - block < 1){
        utils.mem.set('epoch_closed', true);

        // if(store.state.user && store.state.user.isLogin && store.state.user.address !== utils.consts.SUDO_ACCOUNT){
        //   _.delay(async ()=>{
        //     await layer2.user.logout();
        //   }, 5000);
        // }

      }
    });

  }

  getLayer1Instance() {
    if (this.layer1) {
      return this.layer1;
    }

    return null;
  }

  async getCurrentBlock() {
    const rs = this.layer1.queryCurrentBlock()
    return rs.block;
  }

  showQrCodeModal(opts) {
    utils.publish('tea-qrcode-modal', {
      visible: true,
      text: opts.text,
    });
  }
  closeQrCodeModal() {
    utils.publish('tea-qrcode-modal', {
      visible: false,
    });
  }

  blockToDay(block) {
    const hour = 60 * 60 / 12;
    const d = Math.ceil(block / hour);
    if(d < 0) return '0';

    const tmp = moment.utc().preciseDiff(moment.utc().add(d, 'h'), true);
    let rs = '';
    if (tmp.years) {
      rs += tmp.years + 'y';
    }
    if (tmp.months) {
      rs += tmp.months + 'm';
    }
    
    rs += (tmp.days||0) + 'd';

    if(rs === '0d'){
      if(tmp.hours){
        rs = tmp.hours + 'h';
      }
      else if(tmp.minutes){
        rs = tmp.minutes + 'mins'
      }
      else if(tmp.seconds){
        rs = tmp.seconds + 'seconds'
      }
    }

    if(rs === '0d'){
      rs = '0';
    }
    
    return rs;
  }

  encode_b64(str) {
    return forge.util.encode64(str);
  }

  showSelectLayer1Modal() {
    utils.publish('tea-select-layer1-modal', true);
  }

  async getAllDebtByAddress(address){
    const cml_list = await request.layer1_rpc('cml_userCreditList', [
      address
    ]);

    const layer1_instance = this.getLayer1Instance();

    let total = 0;
    const debt_map = {};

    await Promise.all(_.map(cml_list, async (arr)=>{
      const cml_id = arr[0];
      let debt = parseInt(arr[1], 10);
      // let debt = await api.query.cml.genesisMinerCreditStore(address, cml_id);
      // debt = debt.toJSON();
      if (debt) {
        total += debt;
      }
      _.set(debt_map, cml_id, (debt / layer1_instance.asUnit()))
      return null;
    }));

    total = total / layer1_instance.asUnit();

    
    return {
      total,
      details: debt_map
    };

  }

  async getAllPawnByAddress(address){
    return null;
  }

  async getAllBalance(address) {
    // const eth = await this.layer1.getEthBalance();
    const tea = await this.layer1.getTeaBalance();
    // const coffee = await this.layer1.getCoffeeBalance();
    return {
      eth: 0,  //Math.floor(eth * 10000) / 10000,
      free: Math.floor(tea * 10000) / 10000,
      lock: 0,
      reward: 0,
      usd: 0, //coffee,
      usd_debt: 0,
    };
  }

  async transferBalance(address, amount, isCoffee=false) {
    const layer1_account = store.getters.layer1_account;
    if (!layer1_account.address) {
      return false;
    }

    if (!amount || amount === 0) {
      throw 'Invalid transfer balance.';
    }

    if(!address){
      throw 'Invalid receiver\'s address.';
    }

    if(address === layer1_account.address){
      throw 'You cannot send to yourself.';
    }

    const layer1_instance = this.getLayer1Instance();
    const api = layer1_instance.getApi();

    const total = layer1_instance.asUnit() * amount;
    const tt = amount > 9000 ? numberToHex(total) : Math.floor(total);
    console.log(11, tt);
    let transfer_tx = api.tx.balances.transfer(address, tt);
    if(isCoffee){
      transfer_tx = api.tx.genesisExchange.transferUsd(address, tt);
    }
    await layer1_instance.sendTx(layer1_account.address, transfer_tx);
  }

  async getCoupons(address) {
  
    return {
      coupon_investor_A: 0,
      coupon_investor_B: 0,
      coupon_investor_C: 0,
      coupon_team_A: 0,
      coupon_team_B: 0,
      coupon_team_C: 0,
    }
  }

  async refreshCurrentAccount() {

    const layer1_account = store.getters.layer1_account;
    if (!layer1_account.address && !layer1_account.email) {
      return false;
    }


    const balance = await this.getAllBalance(layer1_account.address);

    // reset all state
    store.commit('reset_state');

    // const cml_list = await this.getCmlListByUser(layer1_account.address);
    // const cml_data = await this.getCmlByList(cml_list);
    this._log.i("refresh current layer1_account");
    store.commit('set_account', {
      eth: balance.eth,
      balance: balance.free,
      lock_balance: balance.lock,
      address: layer1_account.address,
      email: layer1_account.email,
      ori_name: layer1_account.name||'_',
      cml: [],
      reward: balance.reward,
      
      usd: balance.usd,
      usd_debt: balance.usd_debt,

      coupons: [],
      pawn_cml_list: [],
    });

    
    await store.dispatch('init_user');
  }

  async getCmlListByUser(address) {
    return await this.layer1.getMyCmlList();
  }

  async getCmlByList(cml_list) {
    return _.map(cml_list, (item)=>{
      item.data = utils.parseJSON(item.uri);
      return item;
    });

  }

  to_default_cml(cml){
    cml.version_expired = false;
    // cml.machine_id = '0x123';
    // cml.remaining_performance = 0;
    // cml.status = '';
    // cml.miner_status = '';
    // cml.staking_slot = 0;
    return cml;
  }




}