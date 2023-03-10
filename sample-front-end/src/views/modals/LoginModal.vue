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

        <h4>Click confirm to login with metamask wallet.</h4>

        <div>
          
        </div>

        

      </div>
    


    </div>
    

    <span slot="footer" class="dialog-footer">

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
      if(cb){
        await cb('sig', this.close);
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