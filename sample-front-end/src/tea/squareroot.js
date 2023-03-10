import utils from './utils';
import {_} from 'tearust_utils';
import {k10, k7} from './kn';

const assert = (exp, val)=>{
  if(exp !== val && exp.toString() !== val.toString()){
    throw `Failed => ${exp} !== ${val}`;
  }
}

const bn_sqrt = (n)=>{
  if(n.eq(utils.toBN(0))) return utils.toBN(0);
  if (n.lte(utils.toBN(3))){
    return utils.toBN(1);
  }
  var z = utils.toBN(0);
  if (n.gt(utils.toBN(3))) {
    z = n;
    var x = n.div(utils.toBN(2)).add(utils.toBN(1));
    while (x.lt(z)) {
      z = x;
      x = n.div(x).add(x).div(utils.toBN(2));
    }
  } else if (n.eq(utils.toBN(0))) {
    z = utils.toBN(1);
  }
  return z;
};

const CENT = utils.toBN('10000000000000000');
const DOLLAR = CENT.mul(utils.toBN(100));

const SquareRoot = class {
  constructor(k){
    this.k = utils.toBN(k);
  }

  pool_balance(balance){
    const b = utils.toBN(balance);
    return bn_sqrt(b).mul(b).div(utils.toBN(100)).mul(utils.toBN(this.k)).mul(utils.toBN(2)).div(utils.toBN('1000000000')).div(utils.toBN(3));
  }
  
  buy_price(total_supply){
    return bn_sqrt(utils.toBN(total_supply)).div(utils.toBN(100)).mul(utils.toBN(this.k)).mul(utils.toBN(1000000000));
  }

  pool_balance_reverse(area, precision){
    const zero = utils.toBN(0);
    area = utils.toBN(area);
    if(area.eq(zero)){
      return zero;
    }

    let _times = 0;
    let last_diff = zero;
		let x_n = this.select_nearest_reference_point(area);

    const diff = (a, b)=>{
			return utils.toBN(a).sub(utils.toBN(b)).abs();;
		};

    while(_times<10000){
      let x_n_plus_1 = null; 
      if(utils.toBN(x_n).lt(utils.toBN(1))){
        x_n_plus_1 = zero;
      } else {
        x_n_plus_1 = 
          area.div(this.k)
          .mul(utils.toBN(1000000000))
          .div(bn_sqrt(x_n))
          .mul(utils.toBN(100))
          // .sub(x_n.mul(utils.toBN(2)).div(utils.toBN(3)))
          .add(x_n.div(utils.toBN(3)));
      }

      if(this.approximately_equals(x_n, x_n_plus_1, precision)){
				return x_n_plus_1;
			} else {
				let new_diff = diff(x_n, x_n_plus_1);
				if( (last_diff.gt(utils.toBN(0))) && (new_diff.gt(last_diff)) ){
					return x_n;
				}
				x_n = x_n_plus_1;
				last_diff = new_diff;
				_times += 1;
			}
    }
    console.log(_times);
  }

  approximately_equals(a, b, precision){
    let abs = utils.toBN(a).sub(utils.toBN(b)).abs();

    return abs.lte(utils.toBN(precision));
  }

  select_nearest_reference_point(area){
		let default_starter = utils.toBN(1100000).mul(utils.toBN(1000000));
    let area_with_k_value = area / this.k * 100;
    const loop = (areas)=>{
      let nearest_index = 0;
      _.each(areas, (item, i)=>{
        if(item > area_with_k_value){
          return false;
        }
        nearest_index = i;
      });

      if(nearest_index < 1){
        return default_starter;
      }
      return utils.toBN(nearest_index*100*DOLLAR);
    };

    switch(this.k.toString()){
      case '10':
        return loop(k10);
      case '7':
        return loop(k7);
      default:
        return default_starter;
    }
	}

};

const F = {
  create(k){
    return new SquareRoot(k);
  },

  calculate_buy_amount(token_amount, total_supply, buy_k){
    const curve = F.create(buy_k);
    const current_pool_balance = curve.pool_balance(total_supply);
    const after_pool_balance = curve.pool_balance(utils.toBN(total_supply).add(utils.toBN(token_amount)));

    return after_pool_balance.sub(current_pool_balance).toString();
  },
  calculate_sell_amount(token_amount, total_supply, sell_k){
    const curve = F.create(sell_k);
    const current_pool_balance = curve.pool_balance(total_supply);
    const after_pool_balance = curve.pool_balance(utils.toBN(total_supply).sub(utils.toBN(token_amount)));

    return current_pool_balance.sub(after_pool_balance).toString();
  },

  test(){
    const verify_pool_balance_and_reverse = ()=>{
      const root_square_100 = F.create(100);
      const xx_list = [
        0,
        1,
        100,
        100000,
        1000000000,
        10000000000,
        100000000000000000000,
        18446744073709551615, //u64 max
      ];
      _.each(xx_list, (total_supply_in_dollar)=>{
        total_supply_in_dollar = utils.toBN(total_supply_in_dollar);
        let total_supply = DOLLAR.mul(total_supply_in_dollar);
        let area = root_square_100.pool_balance(total_supply);
        let reserved_total_supply =
          root_square_100.pool_balance_reverse(area, 1);
        assert(root_square_100.approximately_equals(
          reserved_total_supply,
          total_supply,
          1000
        ), true);
      });

      _.each(xx_list, (total_supply_in_cents)=>{
        total_supply_in_cents = utils.toBN(total_supply_in_cents);
        let total_supply = CENT.mul(total_supply_in_cents);
        let area = root_square_100.pool_balance(total_supply);
        let reserved_total_supply =
          root_square_100.pool_balance_reverse(area, 1);
        assert(root_square_100.approximately_equals(
          reserved_total_supply,
          total_supply,
          1000
        ), true);
      });

    }


    verify_pool_balance_and_reverse();

    return 'Test success.';
  }
};

window.sq_root = F;
export default F;