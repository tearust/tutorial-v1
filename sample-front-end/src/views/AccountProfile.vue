<template>
  <div class="tea-page">
    <el-alert
      v-if="!!top_log"
      effect="dark"
      @close="top_log=null"
      center
      :closable="true"
      :title="top_log"
      style="margin-top:-24px; margin-bottom:20px;"
      type="warning">
    </el-alert>

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
            <!-- <span @click="showAddressQrcode(layer1_account.address)" style="margin-left: 5px;" title="qrcode" class="iconfont tea-icon-btn icon-qr_code"></span> -->
          </span>
        </div>
        <div class="x-item" v-if="layer1_account && !layer1_account.email">
          <b>
            {{ "Chain wallet TEA balance" }}
            <TeaIconButton
              style="position: relative"
              place="right"
              tip="
            The amount of TEA in your layer1 wallet (e.g. Metamask wallet)
          "
              icon="questionmark"
            />
          </b>
          <span
            style="margin-right: 34px"
            :inner-html.prop="
              layer1_account ? layer1_account.balance : '' | teaIcon
            "
          ></span>
          <el-button
            size="mini"
            type="primary"
            plain
            icon="el-icon-refresh"
            circle
            @click="refreshLayer1Balance($event)"
            style="right: 0; position: absolute"
          ></el-button>
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
            {{ "TApp Store wallet TEA deposit" }}
            <!-- <TeaIconButton
              style="position: relative"
              place="right"
              tip="
            The amount of TEA ready to be used in your layer2 wallet. These funds can be transferred gas-free to any TApp where you'd like to use the funds
          "
              icon="questionmark"
            /> -->
          </b>
          <span
            style="margin-right: 34px"
            :inner-html.prop="
              tapp_deposit === null ? '...' : tapp_deposit | teaIcon
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
            style="margin-right: 20px"
            v-if="layer1_account"
            @click="toUniswap()"
          >
            TEA | ETH Exchange
          </el-button>
          
          <el-tooltip
            v-if="layer1_account && !layer1_account.email"
            effect="light"
            placement="top"
            content="Move TEA funds back to your chain wallet(layer1)."
          >
            <el-button :disabled="!tapp_balance" @click="withdrawHandler()"
              >Withdraw</el-button
            >
          </el-tooltip>

          <el-tooltip
            v-if="layer1_account && !layer1_account.email"
            effect="light"
            placement="top"
            content="Move chain wallet (layer1) TEA funds to layer2 TApp Store wallet account."
          >
            <el-button
              style="margin-right: 20px"
              v-if="layer1_account"
              @click="rechargeHandler()"
              >Topup</el-button
            >
          </el-tooltip>
          
          <el-button
            type="primary"
            v-if="user && user.isLogin"
            @click="transferTea()"
          >
            Transfer TEA
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
      // tab: 'my_cml',
      tapp_balance: null,
      tapp_deposit: null,
      top_log: null,
    };
  },
  computed: {
    ...mapGetters(["layer1_account"]),
    ...mapState(['user'])
  },
  async created() {
    this.initCopyEvent();
  },
  beforeDestroy() {
    this.clipboard && this.clipboard.destroy();
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
    showSelectLayer1() {
      this.wf.showSelectLayer1Modal();
    },
    async rechargeHandler() {
      layer2.user.topupFromLayer1(this, async () => {
        this.$root.success("Topup success.");
        // await utils.sleep(2000);
        // this.$root.loading(true, "Refreshing balance ...");
        // await utils.sleep(8000);
        // await this.refreshAccount();
        // this.$root.loading(false);
        await this.smartRefreshBalance();
      });
    },
    async withdrawHandler() {
      try {
        layer2.user.withdrawFromLayer2(this, 1, async () => {
          // await utils.sleep(2000);
          // this.$root.loading(true, "Refreshing balance ...");
          // await utils.sleep(12000);
          // await this.refreshAccount();
          // this.$root.loading(false);
          this.$root.success("Withdraw success.");
          await this.smartRefreshBalance();
        });
      } catch (e) {
        this.$root.showError(e);
      }
    },
    async refreshAccount(flag = false) {
      flag && this.$root.loading(true);
      await this.wf.refreshCurrentAccount();
      if(this.user && this.user.isLogin){
        await this.queryTokenBalance();
        await this.queryDeposit();
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
      this.$root.loading(true, "Refreshing TApp deposit ...");
      await this.queryDeposit();
      this.$root.loading(false);
      e && e.target && e.target.blur();
    },
    async refreshLayer1Balance(e) {
      this.$root.loading(true);
      await this.wf.refreshCurrentAccount();
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
    async queryDeposit() {
      try {
        this.tapp_deposit = await layer2.user.query_deposit(this);
      } catch (e) {
        console.error(e);
      }
    }, 
    async transferTea(){
      await layer2.user.transferTea(this, {}, async ()=>{
        await this.refreshTappBalanceHandler();
      });
    },
    async transferBalance() {
      this.$root.alert_success("Please use Metamask wallet to send");
    },
    async transferUsd() {
      this.$root.alert_success("Please use Metamask wallet to send");
    },
    clickRefreshBtn() {
      utils.publish("refresh-current-account__account");
    },
    showAddressQrcode(address) {
      PubSub.publish("tea-qrcode-modal", {
        info: null,
        visible: true,
        text: address,
      });
    },
    initCopyEvent() {
      const clipboard = new ClipboardJS(".js_copy");
      this.clipboard = clipboard;
      clipboard.on("success", (e) => {
        e.clearSelection();
        this.$root.success("Copied");
      });
      clipboard.on("error", (e) => {});
    },
    toUniswap(){
      const url = 'https://app.uniswap.org/#/pool/45838';
      window.open(url, "_blank");
    },
    async smartRefreshBalance(){
      const max_time = 8;
      const last_layer1 = _.clone(this.layer1_account.balance);
      const last_balance = _.clone(this.tapp_balance);
      let n = 1;
      this.$root.loading(false);
      this.top_log = "Refreshing balance ...";
      const loop = async ()=>{
        await this.wf.refreshCurrentAccount(); 
        await this.queryTokenBalance();
        if(n > max_time){
          this.top_log = null;
          return false;
        }
        if(last_layer1 === this.layer1_account.balance){
          await utils.sleep(5000);
          n++;
          await loop();
        }
        this.top_log = null;
      };
      await loop();
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