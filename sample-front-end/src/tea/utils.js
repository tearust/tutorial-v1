
import Pubsub from 'pubsub-js';
import { ethers } from "ethers";
import * as tearust_utils from 'tearust_utils';
import { 
  hexToString, formatBalance, hexToNumber, hexToBn, numberToHex,
  BN_MILLION, isBn, BN, u8aToHex,

} from '@polkadot/util';

import './index';

import strings from '../assets/string';

const str = (key) => {
  return _.get(strings, key, key);
};

const { _, uuid, forge } = tearust_utils;


const consts = {
  CurveType: {Linear: 'Linear', SquareRoot: 'SquareRoot'},

  gas_tip: (n=1)=>{
    return `This transaction has a ${n} TEA gas fee.`;
  }
};

const _MEM = {};
const mem = {
  set(key, val) {
    _MEM[key] = val;
  },
  get(key) {
    return _.get(_MEM, key, null);
  },
  remove(key) {
    delete _MEM[key];
  }
};

const cache = {
  put(id, data) {
    id = location.pathname+'__'+id;
    localStorage.setItem(id, JSON.stringify(data));
  },
  get(id) {
    id = location.pathname+'__'+id;
    const d = localStorage.getItem(id);
    try {
      return JSON.parse(d);
    } catch (e) {
      return d;
    }
  },
  remove(id) {
    id = location.pathname+'__'+id;
    localStorage.removeItem(id);
  },

};

const layer1 = {
  formatBalance(value, with_icon=false) {
    let is_negative = false;
    if(_.isNumber(value) && value < 0){
      value = Math.abs(value);
      is_negative = true;
    }

    value = F.toBN(value);
    value = F.bnToBalanceNumber(value);
    value = layer1.roundAmount(value);

    if(is_negative){
      return value * -1;
    }

    if(!with_icon) return value;
    const symbol = '<span style="margin-right: 0;" class="iconfont icon-a-TeaProject-T"></span>'
    return symbol + value;

  },
  amountToBalance(value){
    return _.toNumber(value) * (1000000*1000000*1000000);
  },
  balanceToAmount(value){
    return layer1.formatBalance(value);
  },
  roundAmount(value){
    return value;
    // return Math.floor(value*1000000) / 1000000;
  },
  toRealBalance(value){
    value = F.toBN(value);
    value = F.bnToBalanceNumber(value);
    const unit = 1000000*1000000*1000000;
    return Math.floor(value * unit) / unit;
  }
};


const _secret = {
  id: null,
  key : null,
  iv : null,
  hex : null,

  rsa_key: null,
  key_encrypted: null
};
const crypto = {

  get_secret(address){
    address = address || 'NULL';

    const xk = 'crypto-secret-key__'+address;
    if(_secret.id !== xk){
      const __key = localStorage.getItem(xk);
      const key = __key || forge.random.generateSync(16);
      const iv = key;
      const hex = forge.util.bytesToHex(key);

      localStorage.setItem(xk, key);

      _secret.key = key;
      _secret.iv = iv;
      _secret.hex = hex;
      _secret.id = xk;
    }
    return _secret;
  },

  set_rsa_publickey(address, rsa_key){
    crypto.get_secret(address);
    _secret.rsa_key = rsa_key;

    _secret.key_encrypted = crypto.rsaEncodeWithRsaPublickKey(_secret.key, _secret.rsa_key);
  },

  encode(address, buffer_data) {
    const {key, iv} = crypto.get_secret(address);
    const cipher = forge.cipher.createCipher('AES-CBC', key);
    cipher.start({iv: iv});
    cipher.update(forge.util.createBuffer(buffer_data));
    // console.log(111, forge.util.createBuffer(buffer_data))
    cipher.finish();
    const encrypted = cipher.output;

    return encrypted.getBytes();
  },
  decode(address, encryptedBytes) {
    const {key, iv} = crypto.get_secret(address);
    const decipher = forge.cipher.createDecipher('AES-CBC', key);
    decipher.start({iv: iv});
    // const encryptedBytes = forge.util.hexToBytes(hex);
    const length = encryptedBytes.length;

    const chunkSize = 1024 * 64;
    let index = 0;
    let decrypted = '';
    do {
      decrypted += decipher.output.getBytes();
      const buf = forge.util.createBuffer(encryptedBytes.substr(index, chunkSize));
      decipher.update(buf);
      index += chunkSize;
    } while(index < length);
    decipher.finish();
    decrypted += decipher.output.getBytes();
    return decrypted;
  },

  // rsa encode with RSA_PUBLICKEY from step 1
  rsaEncodeWithRsaPublickKey(data, rsa_pub){
    console.log(900, data, rsa_pub);
    const tmp = rsa_pub;
    const pub = forge.pki.publicKeyFromPem(tmp);

    let rs = pub.encrypt(data);

    let xxx = F.stringToU8(rs);
    console.log(903, xxx);
    // console.log(904, F.uint8array_to_base64(xxx));
    // console.log(905, F.stringToU8(rs));
    // return forge.util.encode64(rs);
    return F.uint8array_to_base64(xxx);
  },

  sha256(data) {
    const tmp = forge.sha256.create();
    tmp.update(data);
    return tmp.digest().toHex();
  }
};

const form = {
  nameToLabel(name) {
    return _.map(name.split('_'), (n, i) => {
      if (i > 0) return n;
      return _.capitalize(n);
    }).join(' ');
  }
};

let _http_base_url = '';
const F = {
  cache,
  mem,
  crypto,
  forge,
  layer1,
  consts,
  str,
  form,

  getHttpBaseUrl() {
    if (!_http_base_url) {
      throw 'no http url';

    }

    return _http_base_url;
  },
  setHttpBaseUrl(url) {
    _http_base_url = url;
    http.initBaseUrl();
  },

  convertU8ToString(u8_array) {
    return (_.map(u8_array, (x) => String.fromCharCode(x))).join('');
  },

  uuid() {
    return uuid();
  },

  uint8array_to_arraybuffer(uint8) {
    return uint8.buffer.slice(uint8.byteOffset, uint8.byteOffset + uint8.byteLength);
  },
  uint8array_to_base64(uint8) {
    uint8 = F.convertU8ToString(uint8);
    return forge.util.encode64(uint8);
  },
  stringToU8(str){
    var arr = [];
    for (var i = 0, j = str.length; i < j; ++i) {
      arr.push(str.charCodeAt(i));
    }
  
    var tmpUint8Array = new Uint8Array(arr);
    return tmpUint8Array;
  },
  u8ToString(u8_arr){
    var dataString = "";
    for (var i = 0; i < u8_arr.length; i++) {
      dataString += String.fromCharCode(u8_arr[i]);
    }
  
    return dataString;
  },


  get_env(key) {
    if (key === 'env') {
      return process.env.NODE_ENV;
    }

    const x_key = 'VUE_APP_' + _.toUpper(key);
    return _.get(process.env, x_key, null);
  },
  is_dev(){
    return process.env.NODE_ENV === 'dev';
  },


  register: (key, cb) => {
    Pubsub.unsubscribe(key);
    Pubsub.subscribe(key, cb);
  },
  publish: Pubsub.publish,

  async sleep(time) {
    return new Promise((resolve) => setTimeout(resolve, time))
  },

  toNumber(n) {
    const tmp = n.toString().replace(/,/g, '');
    return _.toNumber(tmp);
  },

  toBN(val){
    if(isBn(val)) return val;
    if(_.isNumber(val)){
      return hexToBn(numberToHex(val));
    }
    if(_.isString(val)){
      if(_.startsWith(val, '0x')){
        return hexToBn(val);
      }

      return new BN(val);
    }

    throw 'Can not convert to BN => '+val;
  },

  urlHashParam(key){
    const l = _.last(location.hash.split('?'));
    let tmp = l.split('&');
    tmp = _.map(tmp, (arr)=>{
      const t = arr.split('=');
      return {
        key: t[0],
        value: t[1],
      }
    });

    const rs =  _.find(tmp, (x)=>x.key === key);
    return rs ? rs.value : null;
  },
  urlParam(key){
    const l = location.search.replace(/^\?/, '');
    let tmp = l.split('&');
    tmp = _.map(tmp, (arr)=>{
      const t = arr.split('=');
      return {
        key: t[0],
        value: t[1],
      }
    });

    const rs =  _.find(tmp, (x)=>x.key === key);
    return rs ? rs.value : null;
  },

  parseJSON(str, default_value=null){
    let rs;
    try{
      rs = JSON.parse(str);
    }catch(e){
      rs = default_value
    }

    return rs;
  },

  bnToBalanceNumber(bn){
    const value = ethers.utils.formatUnits(bn.toString(), 'ether');
    // const value = Number(BigInt(bn.toString())/1000000000000000000n);
    return _.toNumber(value);
  },



  rpcArrayToString(arr){
    return hexToString(u8aToHex(arr));
  },

  urlToLink(url){
    if(!_.startsWith(url, 'https://') && !_.startsWith(url, 'http://')){
      url = 'http://'+url; 
    }

    return url;
  },

  urlParam(key){
    const l = location.search.replace(/^\?/, '');
    let tmp = l.split('&');
    tmp = _.map(tmp, (arr)=>{
      const t = arr.split('=');
      return {
        key: t[0],
        value: t[1],
      }
    });

    const rs =  _.find(tmp, (x)=>x.key === key);
    return rs ? rs.value : null;
  },


  isValidIP(ip){
    var reg = /^(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])$/
    return reg.test(ip);
  },

  minerHexToB64(hex){
    if(!hex) return '';
    const val = hex.replace(/^0x/, '');
    return forge.util.encode64(forge.util.hexToBytes(val));
  },
  u8aToB64(u8a){
    return F.minerHexToB64(u8aToHex(Uint8Array.from(u8a)));
  },
  B64ToHex(b64){
    return '0x'+forge.util.bytesToHex(forge.util.decode64(b64));
  },
  randomTappId(){
    const rs = [];
    for(let i=0; i<32; i++){
      rs.push(_.random(0, 255));
    }
    return rs;
  },

  tappLayer2FormatError(error){
    if(_.includes(error, 'not_login')){
      return [true, 'not_login', '', '', ''];
    }
    error = _.trim(error);
    return [false, error];

  },

  getEpochEndBlock(){
    return _.toNumber(F.get_env('end_block')||'20000000');
  },

  nextTick(cb){
    setTimeout(cb, 1);
  },

  emailToAddress(email){
    const str = F.crypto.sha256(_.toLower(email));
    return '0x'+str.substring(0, 40);
  },

  isEmail(email){
    const reg = /^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
    return reg.test(email);
  },


};

window.utils = F;
export default F;