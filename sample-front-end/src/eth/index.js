import { ethers, BigNumber } from "ethers";
import utils from '../tea/utils';
import { ChainMap, ContractMap } from "./consts";
import {_, moment} from 'tearust_utils';
import store from '../store';
import help from './help';

const U = ethers.utils;

class Instance {
  constructor(){
    this.cache = {
      block: null,

      account_list: null,
    };

    this.provider = new ethers.providers.Web3Provider(window.ethereum);

    this.maintainer_contract = new ethers.Contract(
      ContractMap.MAINTAINER, 
      require('./abi/Maintainer.sol/Maintainer.json').abi, 
      this.provider.getSigner(),
    );

    this.lock_contract = new ethers.Contract(
      ContractMap.LOCK,
      require('./abi/Lock.sol/Lock.json').abi,
      this.provider.getSigner(),
    );

    this.tea_contract = new ethers.Contract(
      ContractMap.ERC20,
      require('./abi/ERC20.sol/ERC20Token.json').abi,
      this.provider.getSigner(),
    );
    this.cml_contract = new ethers.Contract(
      ContractMap.ERC721,
      require('./abi/ERC721.sol/ERC721.json').abi,
      this.provider.getSigner(),
    );

    this.coffee_contract = new ethers.Contract(
      ContractMap.COFFEE,
      require('./abi/Coffee.sol/COFFEE.json').abi,
      this.provider.getSigner(),
    );

    this.token_vesting_contract = new ethers.Contract(
      ContractMap.TOKENVESTING,
      require('./abi/TokenVesting.sol/TokenVesting.json').abi,
      this.provider.getSigner(),
    );

    this.signer = this.provider.getSigner();

    this._init = 0;
  }
  async init(){
    await this.initCache();
    await this.initEvent();

  }

  async initCurrentAccount(){
    await this.connect();
    const acct = await this.getAccountList();
    console.log('current account =>', acct);

    this._init = 1;
    return _.first(acct);
  }

  async getAccountList(){
    return await this.provider.listAccounts();
  }

  async initEvent(){
    this.provider.on('block', (block)=>{
      this.cache.block = block;
      utils.publish('layer1_block', {block});
    });

    window.ethereum.on('accountsChanged', function (accounts) {
      console.log('Wallet account changed =>', accounts);
      location.reload(true);
    });
  }

  async initCache(){
    const bb = await this.queryCurrentBlock();
    this.cache.block = bb.block;
    store.commit('set_chain', {
      current_block: bb.block
    });

    this.cache.account_list = await this.getAccountList();
  }

  async connect(){
    await this.provider.send("eth_requestAccounts", [])
  }

  async requestWalletAddressList(){
    const list = await this.provider.send("eth_requestAccounts", []);
    console.log('requestWalletAddressList:', list);
    return list;
  }
  async getEthBalance(){
    const n = await this.signer.getBalance();
    const balance = U.formatUnits(n, 'ether');
    return balance;
  }
  async getTeaBalance(){
    const erc20Token = this.tea_contract;
    const me = await this.signer.getAddress();
    const n = await erc20Token.balanceOf(me);
    const balance = U.formatUnits(n, 'ether');
    return balance
  }
  async getCoffeeBalance(){
    // const n = await this.coffee_contract.balanceOf(this.signer.getAddress());
    // const balance = U.formatUnits(n, 'ether');
    // return balance
    return 0;
  }

  async getChain(){
    const cid = await this.signer.getChainId();
    const cname = _.get(ChainMap, cid, 'UnknownChainId');
    return {
      id: cid,
      name: cname,
    };
  }

  async topup(amt){

    const erc20Token = this.tea_contract;
    const lock = this.lock_contract;
    const signer = this.signer;

    const current_address = await signer.getAddress();
    // await erc20Token.approve(lock.address, help.unit(100));
       
    // lock
    const types = {
      Permit: [
        { name: "owner", type: "address" },
        { name: "spender", type: "address" },
        { name: "value", type: "uint256" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
      ],
    };
    const chainId = (await this.getChain()).id;
    const domain = {
      name: "TEA Project",
      version: "1",
      chainId: chainId,
      verifyingContract: erc20Token.address,
    };
    const deadline = parseInt(new Date().getTime() / 1000) + 10000;
    const amount = help.unit(amt);
    const value = {
      owner: current_address,
      spender: lock.address,
      value: amount,
      nonce: await erc20Token.nonces(current_address),
      deadline,
    };

    const signature = await signer._signTypedData(domain, types, value);
    const r = '0x' + signature.substring(2).substring(0, 64);
    const s = '0x' + signature.substring(2).substring(64, 128);
    const v = '0x' + signature.substring(2).substring(128, 130);
    const res = await lock.TopupWithPermit(
      erc20Token.address,
      amount,
      deadline,
      v,
      r,
      s,
      false
    );
    console.log('result:', res);
    
    return true;
  }

  async signMessage(message){
    await this.connect();
    const msg1 = (U.toUtf8Bytes(message));
    // const signature = await this.signer._legacySignMessage(U.arrayify(msg1));
    const signature = await this.signer.signMessage(msg1);
    const pk = U.recoverPublicKey(U.hashMessage(msg1), signature);
    const sig = signature

    return [sig, pk, U.toUtf8Bytes(message), message];
  }

  async getMaintainerAddressList(){
    // return await this.maintainer_contract.listValidators();
    return await this.provider.call({
      to: '0x368e02679706fbC77ff1173aa1a87AED35B22507',
      data: '0x3b3b57debf074faa138b72c65adbdcfb329847e4f2c04bde7f7dd7fcad5a52d2f395a558'
    }, 'latest');
  }

  async queryCurrentBlock(){
    const rs = await this.provider.getBlock();
    return {
      block: rs.number,
      difficulty: rs.difficulty,
      time: new Date(rs.timestamp*1000)
    }
  }
  async getCurrentBlock(){
    return this.cache.block;
  }

  isConnected(){
    if(this._init > 1) return 2;
    if(this._init === 1){
      this._init++;
      return 1;
    }
    return this._init;
  }

  async getMyCmlList(){
    const total = (await this.cml_contract.totalSupply()).toString();
    const list = [];
    const me = await this.signer.getAddress();
    for(let i=0; i<total; i++){
      const owner = await this.cml_contract.ownerOf(i);
      if(owner === me){
        list.push({
          id: i,
          owner,
          uri: await this.cml_contract.tokenURI(i)
        })
      }
      
    }
    return list;
  }

  async scheduleListForVesting(address){
    address = address || this.signer.getAddress();
    const rs = await this.token_vesting_contract.getVestingSchedulesCountByBeneficiary(address);
    const list = []

    const day = 3600*24;
    for(let i=0; i<rs.toNumber(); i++){
      const sid = await this.token_vesting_contract.computeVestingScheduleIdForAddressAndIndex(address, i);

      const xxx = await this.token_vesting_contract.computeReleasableAmount(sid);
      const details = await this.token_vesting_contract.getVestingSchedule(sid);

      const dur = details.duration.toNumber();
      const per = details.slicePeriodSeconds.toNumber();
      
      const item = {
        index: i,
        schedule_id: sid,
        amount: utils.layer1.balanceToAmount(xxx.toString()),
        details,
        info: {
          total: utils.layer1.balanceToAmount(details.amountTotal.toString()),
          released: utils.layer1.balanceToAmount(details.released.toString()),
          available: utils.layer1.balanceToAmount(details.amountTotal.sub(details.released).sub(xxx).toString()),
          start: moment.utc(details.start.toNumber()*1000).format('MMM Do, YYYY'),
          duration: dur < day ? '< 1d' : (Math.floor(dur/day)+'d'),
          cliff: moment.utc(details.cliff.toNumber()*1000).format('MMM Do, YYYY'),
          period: per < day ? '< 1d' : (Math.floor(per/day)+'d'),
        },
      };

      list.push(item);
    }
    console.log('token_vesting list =>', list);
    return list;
  }
  async releaseTeaForVesting(schedule_id, amount){
    await this.token_vesting_contract.release(schedule_id, amount);
    return true;
  }

  async test(){
    // console.log(this.maintainer_contract)
    const list = await this.maintainer_contract.listValidators();
    console.log(1, list);
    return list;
  }
}

const Empty_Instance = class {
  constructor(){}
  initCurrentAccount(){
    return null;
  }
  getChain(){
    return {
      id: 10000,
      name: 'Offline'
    }
  }
  isConnected(){
    return 2;
  }
  getTeaBalance(){
    return 0;
  }
};

let instance = null;
export default {
  help,
  async get(){
    if(instance) return instance;
    try{
      instance = new Instance();
      await instance.init();
      return instance;
    }catch(e){
      instance = new Empty_Instance();
      return instance;
    }
    
  },
  
};