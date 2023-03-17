<template>
<div class="tea-page">
  <h4>Admin -- Init TApp</h4>


  <div class="tea-legend" style="
    margin-top: 40px;
    position: relative;
  ">

    <el-button style="width:400px;position:absolute;top:0; left:0;" type="primary" @click="init_token()">Init TApp token</el-button>

    <el-button style="width:400px;position:absolute;top:0; right:0;" type="primary" @click="init_db()">Init TApp db</el-button>
  </div>
  
  

</div>
</template>
<script>
import Base from '../workflow/Base';
import {_} from 'tearust_utils';
import utils from '../tea/utils';
import { mapGetters, mapState } from 'vuex';
import TeaTable from '../components/TeaTable';
import TeaTableColumn from '../components/TeaTableColumn';
import TeaIconButton from '../components/TeaIconButton';
import layer2 from '../layer2';
export default {
  components: {
    TeaTable,
    TeaIconButton,
    TeaTableColumn,
  },
  data(){
    return {
      list: null,
    }
  },
  computed: {
    ...mapGetters([
      'layer1_account'
    ]),
    ...mapState(['user']),
  },
  async mounted(){
    this.wf = new Base();
    await this.wf.init();
  },
  methods: {
    async init_db(row){
      try{
        await layer2.task.initDB(this, async (rs)=>{
        });
      }catch(e){
        this.$root.showError(e);
      }
    },
    async init_token(row){
      try{
        await layer2.task.initToken(this, async (rs)=>{
        });
      }catch(e){
        this.$root.showError(e);
      }
    },
    
    
  }
};
</script>
<style lang="scss">
</style>