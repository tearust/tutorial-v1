<template>
  <el-dialog
    title="Login"
    :visible="visible"
    width="70%"
    :close-on-click-modal="false"
    custom-class="tea-modal"
    :destroy-on-close="true"
    @opened="openHandler()"
    @close="close()"
  >

    <i v-if="!param || loading" class="el-icon-loading" style="display: block; width: 40px; height: 40px;font-size: 40px; margin: 0 auto;"></i>

    <div v-if="!loading" style="text-align:left;">
      <div style="font-size: 15px;" v-if="layer1_account.address">

        <h4>Please confirm the following permissions:</h4>

        <div>
          
        </div>

        <div>
          <div>
            <el-checkbox v-model="move">Move</el-checkbox>
            <span style="margin-left: 30px;">Allow this app to transfer your funds to other users.</span>
          </div>
          <div style="margin-top:4px;">
            <el-checkbox v-model="consume">In-app purchases</el-checkbox>
            <span style="margin-left: 30px;">Allow this app to spend your funds for app-related functions.</span>
          </div>
          <div style="margin-top:4px;">
            <el-checkbox v-model="bonding_curve">Manage investments</el-checkbox>
            <span style="margin-left: 30px;">Allow this app to buy, sell, or transfer your token assets.</span>
          </div>
          <div style="margin-top:4px;">
            <el-checkbox v-model="withdraw">Withdraw</el-checkbox>
            <span style="margin-left: 30px;">Allow this app to move funds from TEA's layer-2 to Ethereum's layer-1 blockchain.</span>
          </div>

        </div>

        <ul style="margin-top: 12px; list-style:none;padding:0;">
          <li>These permissions are necessary for the TApp to perform actions on behalf of the user.</li>
          
        </ul>

      </div>
      <!-- <el-button v-if="layer1_account.address" type="primary" @click="confirm()">Login</el-button> -->

      <p style="font-size: 16px; color: #f00;" v-if="!layer1_account.address">
        Please select account from Metamask wallet.
      </p>
    </div>
    

    <span slot="footer" class="dialog-footer">
      <!-- <el-button style="float:left;" size="small" @click="close(); $root.goPath('/login_with_email')" type="primary">No wallet? login with email</el-button> -->

      <el-button size="small" @click="close()">Cancel</el-button>
      <el-button size="small" type="primary" @click="confirm()">Login</el-button>
    </span>

  </el-dialog>


</template>
<script>
import { mapState, mapGetters } from 'vuex';
import store from '../../store/index';
import utils from '../../tea/utils';
import Base from '../../workflow/Base';
import {_} from 'tearust_utils';
import layer2 from '../../layer2';
export default {
  data(){
    return {
      loading: true,
      form: {
        
      },
      
      read: false,
      withdraw: false,
      consume: true,
      move: true,
      bonding_curve: false,
    };
  },
  computed: {
    ...mapGetters([
      'layer1_account'
    ]),
    ...mapState('modal', {
      visible:state => store.state.modal.login.visible,
      param: state => store.state.modal.login.param,
    })
  },
  methods: {
    reset(){
      this.loading = true;
      this.form = {
        
      };
    },
    close(){
      this.$store.commit('modal/close', 'login');
      _.delay(()=>{
        this.reset();
      }, 500);
    },
    async confirm(){
      const cb = utils.mem.get('login');
      const tmp = [];
      if(this.read) tmp.push('read');
      if(this.move) tmp.push('move');
      if(this.withdraw) tmp.push('withdraw');
      if(this.consume) tmp.push('consume');
      if(this.bonding_curve) tmp.push('bonding_curve');
  
      if(cb){
        await cb('sig_'+tmp.join("_"), this.close);
      }
    },
    async openHandler(){
      this.wf = new Base();
      await this.wf.init();
      this.loading = false;
    }
  }
}
</script>