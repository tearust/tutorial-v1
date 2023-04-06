<template>
<div class="tea-page">
  <h4>Logs : {{showDate(current_date)}}</h4>

  <div>
    <el-switch
      v-if="user && user.isLogin"
      v-model="mine"
      active-color="#35a696"
      inactive-color="#409eff"
      active-text="Mine"
      :width="60"
      inactive-text="All"
      @change="changeHandler()"
    >
    </el-switch>

  </div>

  <el-button size="small" class="tea-refresh-btn" type="primary" plain icon="el-icon-refresh" circle @click="refreshPage()"></el-button>


  <div>
    <el-button v-for="(d, i) of date_list" :key="i" 
      :disabled="showDate(current_date)===showDate(d)"
      @click="query_op_logs(d)"
      type="text">
      {{showDate(d)}}
    </el-button>
  </div>
  <TeaTable
    :data="list || []"
    name="logs_table"
  >

    <el-table-column
      prop="time"
      label="UTC"
    />
    <el-table-column
      prop="account"
      label="Account"
    />
    <el-table-column
      prop="token_id"
      label="Token ID"
    />
    <el-table-column
      prop="state_type"
      label="Token type"
    />
    <el-table-column
      prop="statement_type"
      label="Transcation type"
    />

    <TeaTableColumn
      label="Token quantity"
      width="120"
      tip="Units are either Tea token or corresponding entity token."
    >
      <template slot-scope="scope">
        <span v-if="scope.row.state_type==='Tea'" :inner-html.prop="scope.row.gross_amount | balance"></span>
        <span v-if="scope.row.state_type!=='Tea'" :inner-html.prop="scope.row.gross_amount | balance_number"></span>
      </template>
    </TeaTableColumn>

    <el-table-column
      prop="memo"
      label="Memo"
    />

    
  </TeaTable>
  

</div>
</template>

<script>
import {_, moment} from 'tearust_utils';
import utils from '../tea/utils';
import { mapState, mapGetters } from 'vuex';
import TeaTable from '../components/TeaTable';
import TeaTableColumn from '../components/TeaTableColumn';
import layer2 from '../layer2';
export default {
  components: {
    TeaTable,
    TeaTableColumn,
  },
  data(){
    return {
      current_date: null,
      address: null,
      date_list: [],
      list: null,
      mine: true,
    };
  },
  computed: {
    ...mapGetters([
      'layer1_account'
    ]),
    ...mapState([
      'user'
    ]),
  },
  async mounted(){
    const xl = [];
    const today = moment.utc();
    xl.push(today.clone());
    for(let i=0; i<9; i++){
      const tmp = today.subtract(1, 'days');
      xl.push(tmp.clone());
    }
    this.date_list = xl;
    this.address = this.layer1_account.address;
    this.current_date = moment.utc();
    if(this.user && this.user.isLogin){
      this.mine = true;
    }
    else{
      this.mine = false;
    }
    await this.refreshPage();
    
  },
  methods: {
    async refreshPage(){
      this.$root.loading(true);
      const d = this.current_date;
      const param = {
        day: d.date(),
        year: d.year(),
        month: d.month()+1,
      };
      if(this.mine){
        param.address = this.address;
      }
      const list = await layer2.log.queryOpLogs(this, param);
      this.list = list;
      this.$root.loading(false);
    },
    showDate(c){
      if(!c) return moment.utc().format('YYYY/MM/DD');
      return moment.utc(c).format('YYYY/MM/DD');
    },
    async query_op_logs(date=moment.utc()){
      this.current_date = date;
      await this.refreshPage();
    },
    async changeHandler(){
      await this.refreshPage();
    },
    
  }
  
}
</script>