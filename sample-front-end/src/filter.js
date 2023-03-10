import Vue from 'vue';
import utils from './tea/utils';
import { _ } from 'tearust_utils';
import strings from './assets/string';


Vue.filter('formatBalance', (value) => {
  if (!value) return '';
  return utils.layer1.formatBalance(value);
});

Vue.filter('addTea', (value) => {
  return `${value} TEA`;
});
Vue.filter('teaIcon', (value=0) => {
  const symbol = '<span style="margin-right: 0;" class="iconfont icon-a-TeaProject-T"></span>'
  return symbol + (_.isNull(value)?'0':value);
});
Vue.filter('usd', (value)=>{
  const symbol = '<span style="margin-right: 4px;" class="iconfont icon-kafei"></span>'
  return symbol + (_.isNull(value)?'0':value);
})
Vue.filter('balance', (value) => {
  if(_.isNull(value) || _.isUndefined(value)) return '';
  // value = parseInt(value, 10) / (1000000*1000000);
  // value = Math.floor(value*10000)/10000;

  // const symbol = '<span style="margin-right: 0;" class="iconfont icon-a-TeaProject-T"></span>'
  // return symbol + value;
  return utils.layer1.formatBalance(value, true);
});

Vue.filter('bn_balance', (bn_value)=>{
  if(_.isNull(bn_value) || _.isUndefined(bn_value)) return '';
  return utils.layer1.formatBalance(bn_value, true);
});


Vue.filter('balance_number', (value) => {
  if(_.isNull(value) || _.isUndefined(value)) return '';
  return utils.layer1.formatBalance(value);

});
Vue.filter('cardTitle', (value) => {
  return value.split(' ').join(' ');
});
Vue.filter('str', (key) => {
  return _.get(strings, key, key);
});

Vue.filter('upper', (value)=>{
  return _.toUpper(value);
});

Vue.filter('percent', (value)=>{
  return (value*100) + '%';
});

Vue.filter('minerHexToB64', (hex)=>{
  return utils.minerHexToB64(hex);
});

Vue.filter('erc20', (hex)=>{
  const USDT_ADDRESS = '0xdac17f958d2ee523a2206206994597c13d831ec7';
  hex = USDT_ADDRESS;
  const html = '<a style="overflow:hidden;text-overflow: ellipsis;white-space:nowrap;color:#35A696;" href="https://etherscan.io/token/'+hex+'" target="_blank">'+hex+'</a>';
  return html;
});