<template>
  <div class="tea-page">


    <div class="tea-card">
      <i class="x-icon">
        <img src="fav.png" />
      </i>

      <div class="x-list" style="width: 100%">
        
        <div class="x-item">
          <b>{{ "Address" | cardTitle }}</b>
          <span>
            <font class="js_need_copy">{{
              layer1_account ? layer1_account.address : ""
            }}</font>
            <span
              title="copy"
              data-clipboard-target=".js_need_copy"
              style="margin-left: 5px"
              class="iconfont tea-icon-btn icon-copy js_copy"
            ></span>
            
          </span>
        </div>
        

        <div class="x-item">
          <b>
            {{ "TApp Store wallet TEA balance" }}
            <TeaIconButton
              style="position: relative"
              place="right"
              tip="
            The amount of TEA ready to be used in your layer2 wallet. These funds can be transferred gas-free to any TApp where you'd like to use the funds
          "
              icon="questionmark"
            />
          </b>
          <span
            style="margin-right: 34px"
            :inner-html.prop="
              tapp_balance === null ? '...' : tapp_balance | teaIcon
            "
          ></span>

          <el-button
            size="mini"
            type="primary"
            plain
            icon="el-icon-refresh"
            circle
            @click="refreshTappBalanceHandler($event)"
            style="right: 0; position: absolute"
          ></el-button>
        </div>

        <div class="x-item">
          <b>
            {{ "Current TApp spending limit" }}
            <TeaIconButton
              style="position: relative"
              place="right"
              tip="
            The limit on the total transaction value that can be sent through this TApp. This amount can be changed in the TApps tab of the TAppStore wallet.
          "
              icon="questionmark"
            />
          </b>
          <span
            style="margin-right: 34px"
            :inner-html.prop="
              tapp_allowance === null ? '...' : tapp_allowance | teaIcon
            "
          ></span>

          <el-button
            size="mini"
            type="primary"
            plain
            icon="el-icon-refresh"
            circle
            @click="refreshTappDepositHandler($event)"
            style="right: 0; position: absolute"
          ></el-button>
        </div>

        
        <div class="x-bottom">
          
          <el-button
            type="primary"
            @click="faucet()"
          >
            Faucet TEA
          </el-button>

          <el-button
            type="primary"
            @click="setAllowance()"
          >
            Set spend limit
          </el-button>

        </div>
        
      </div>
    </div>

  </div>
</template>
<script>
import SettingAccount from "../workflow/Base";
import { _ } from "tearust_utils";
import utils from "../tea/utils";
import { mapGetters, mapState } from "vuex";
import PubSub from "pubsub-js";
import ClipboardJS from "clipboard";
import TeaIconButton from "../components/TeaIconButton";
import layer2 from "../layer2";
export default {
  components: {
    TeaIconButton,
  },
  data() {
    return {
      tapp_balance: null,
      tapp_allowance: null,
    };
  },
  computed: {
    ...mapGetters(["layer1_account"]),
    ...mapState(['user'])
  },
  async created() {
    
  },
  beforeDestroy() {
    
  },
  async mounted() {
    layer2.base.set_global_log(this);
    this.$root.loading(true);
    await utils.sleep(1500);
    this.wf = new SettingAccount();
    await this.wf.init();
    await this.refreshAccount();
    this.$root.loading(false);

    utils.register(
      "refresh-current-account__account",
      async (key, param = {}) => {
        await this.refreshAccount();
      }
    );

  },
  methods: {
    
    async refreshAccount(flag = false) {
      flag && this.$root.loading(true);
      if(this.user && this.user.isLogin){
        await this.queryTokenBalance();
        this.tapp_allowance = await layer2.user.query_current_allowance(this, true);
      }
      
      flag && this.$root.loading(false);
    },
    async refreshTappBalanceHandler(e) {
      this.$root.loading(true, "Refreshing TApp balance ...");
      await this.queryTokenBalance();
      this.$root.loading(false);
      e && e.target && e.target.blur();

    },
    async refreshTappDepositHandler(e) {
      this.$root.loading(true, "Refreshing TApp spend limit ...");
      this.tapp_allowance = await layer2.user.query_current_allowance(this, true);
      this.$root.loading(false);
      e && e.target && e.target.blur();
    },

    async queryTokenBalance() {
      try {
        this.tapp_balance = await layer2.user.query_balance(this);
      } catch (e) {
        console.error(e);
      }
    },

    
    clickRefreshBtn() {
      utils.publish("refresh-current-account__account");
    },
    
    async faucet(){
      await layer2.user.faucet(this, {}, async ()=>{
        await this.refreshAccount();
      });
    },
    async setAllowance(){
      await layer2.tapp.setAllowance(this, {}, async ()=>{
        await this.refreshAccount();
      });
    }
    
  },
};
</script>

<style lang="scss">
.tea-page {
  .t-major-financial {
    margin-top: 5px;
    text-align: right;
    padding-right: 8px;
    b {
      color: #35a696;
    }
    span {
      margin: 0 5px;
      color: #c9c9c9;
    }
    span.iconfont {
      color: #35a696;
      margin: 0;
    }
  }
}
</style>