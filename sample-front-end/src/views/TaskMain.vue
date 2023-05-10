<template>
<div class="tea-page">
  <h4>Retweet task list</h4>

  <el-button size="small" class="tea-refresh-btn" type="primary" plain icon="el-icon-refresh" circle @click="refreshList()"></el-button>
  <TeaTable
    :data="list || []"
    name="task_list_table"
  >
    <el-table-column
      prop="subject"
      label="Tweet Id"
    />

    <el-table-column
      prop="creator"
      label="Creator"
    />

    <el-table-column
      label="Price"
    >
      <template slot-scope="scope">
        <span :inner-html.prop="scope.row.price | teaIcon"></span>
      </template>
    </el-table-column>

    <el-table-column
      label="Required deposit"
    >
      <template slot-scope="scope">
        <span :inner-html.prop="scope.row.deposit | teaIcon"></span>
      </template>
    </el-table-column>

    <el-table-column
      prop="worker"
      label="Worker"
    />

    <el-table-column
      prop="status"
      label="Status"
    />

    <el-table-column
      label="Actions"
      width="200"
      fixed="right"
    >
      <template slot-scope="scope">
        

        <TeaIconButton v-if="!$root.eq(scope.row.creator, user.address) && scope.row.status==='New'" tip="Take task" icon="NA" title="Take" @click="takeTask(scope.row)" />
        <TeaIconButton v-if="$root.eq(scope.row.worker, user.address) && scope.row.status==='InProgress'" tip="Complete task" icon="NA" title="Complete" @click="completeTask(scope.row)" />



        <TeaIconButton v-if="$root.eq(scope.row.creator, user.address) && (scope.row.status==='New' || scope.row.status==='Done')" tip="Remove task" icon="NA" title="Remove" @click="deleteTask(scope.row)" />
      </template>
    </el-table-column>


  </TeaTable>

  <div class="tea-legend" style="
    margin-top: 40px;
    position: relative;
  ">

    <el-button style="width:400px;position:absolute;top:0; right:0;" type="primary" @click="createNewTask()">Create new task</el-button>
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
    await this.refreshList();
  },
  methods: {
    async refreshList(){
      this.$root.loading(true);
      const list = await layer2.task.queryTaskList(this);
      this.list = list;
      this.$root.loading(false);
    },
    
    async createNewTask(){
      try{
        await layer2.task.createNewTask(this, {}, async (rs)=>{
          this.$root.success("create task success");
          await this.refreshList();
        });
      }catch(e){
        this.$root.showError(e);
      }
    },
    async deleteTask(row){
      try{
        await layer2.task.deleteTask(this, row, async (rs)=>{
          this.$root.success("delete task success");
          await this.refreshList();
        });
      }catch(e){
        this.$root.showError(e);
      }
    },
    async completeTask(row){
      try{
        await layer2.task.completeTask(this, row, async (rs)=>{
          this.$root.success("The task bounty has been deposited into your wallet.");
          await utils.sleep(8000);
          await this.refreshList();
        });
      }catch(e){
        this.$root.showError(e);
      }
    },
    async takeTask(row){
      try{
        await layer2.task.takeTask(this, row, async (rs)=>{
          this.$root.success("Take task success");
          await this.refreshList();
        });
      }catch(e){
        this.$root.showError(e);
      }
    },
    // async verifyTask(row, ok){
    //   try{
    //     await layer2.task.verifyTask(this, {...row, ok}, async (rs)=>{
    //       this.$root.success("Verify task success");
    //       await this.refreshList();
    //     });
    //   }catch(e){
    //     this.$root.showError(e);
    //   }
    // }
    
  }
};
</script>
<style lang="scss">
</style>