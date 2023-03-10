<template>

  <el-dialog
    :visible="visible"
    width="78%"
    :title="title"
    :close-on-click-modal="false"
    custom-class="tea-modal"
    :destroy-on-close="true"
    @opened="openHandler()"
    @close="close()"
  >
    <!-- <div slot="title" class="el-dialog__title" v-html="title"></div> -->

    <i v-if="!param || loading" class="el-icon-loading" style="display: block; width: 40px; height: 40px;font-size: 40px; margin: 0 auto;"></i>

    <p v-if="!loading && param.text" class="c-info" :inner-html.prop="param.text"></p>
    
    <el-form v-if="!loading" ref="tx_form" :model="form" :rules="rules" :label-width="(param.label_width||150)+'px'">

      <div style=""
        v-for="(item) in props" 
        :key="item.name" 
      >
      <el-form-item 
        v-if="!props[item.name].condition || props[item.name].condition.value===form[props[item.name].condition.target]"
        :label="labels[item.name]" 
        :prop="item.name" 
        :class="(props[item.name] && props[item.name].class) || ''"
      >
        <el-input v-if="types[item.name]==='Input'" :disabled="props[item.name].disabled||false" 
          v-model="form[item.model||item.name]" v-bind="{...props[item.name].el_props||{}}"></el-input>

        <el-select v-if="types[item.name]==='select'" v-model="form[item.name]" placeholder="Please select." v-bind="{...props[item.name].el_props||{}}">
          <el-option
            v-for="item in props[item.name].options || []"
            :key="item.key || item.id"
            :label="item.label || item.id"
            :value="item.value || item.id"
          >
          </el-option>
        </el-select>
        <el-select v-if="types[item.name]==='select_number'" v-model.number="form[item.name]" placeholder="Please select." v-bind="{...props[item.name].el_props||{}}">
          <el-option
            v-for="item in props[item.name].options || []"
            :key="item.key || item.id"
            :label="item.label || item.id"
            :value="item.value || item.id"
          >
          </el-option>
        </el-select>

        <el-input-number v-if="types[item.name]==='number'" :disabled="props[item.name].disabled||false" v-model="form[item.name]" :min="props[item.name].min || 0" :max="props[item.name].max || 2000000" :step="props[item.name].step || 1" v-bind="{...props[item.name].el_props||{}}" style="width: 200px;"></el-input-number>

        <el-checkbox v-if="types[item.name]==='checkbox'" v-model="form[item.name]" v-bind="{...props[item.name].el_props||{}}"></el-checkbox>

        <el-radio-group v-if="types[item.name]==='radio-group'" v-model="form[item.name]" v-bind="{...props[item.name].el_props||{}}">
          <el-radio-button 
            v-for="item in props[item.name].options || []"
            :disabled="item.disabled || false"
            :key="item.key || item.id"
            :label="item.value || item.id"
          >
            {{item.label || item.id}}
          </el-radio-button>
        </el-radio-group>

        <el-switch v-if="types[item.name]==='switch'" v-model="form[item.name]" v-bind="{...props[item.name].el_props||{}}">
        </el-switch>



        <div class="t-action" v-if="props[item.name].after">
          <span class="s1" :inner-html.prop="props[item.name].after"></span>
        </div>

        <TeaIconButton style="margin-left: 10px;" icon_style="font-size:18px;" v-if="props[item.name].tip" :tip="props[item.name].tip" @click="props[item.name].tip_action ? props[item.name].tip_action() : ()=>{}" icon="questionmark" />

        <div class="t-action" v-if="props[item.name].action">
          <span class="s1" v-if="props[item.name].action.html" :inner-html.prop="props[item.name].action.html"></span>
          <span class="s1" v-if="props[item.name].action.tip_html" :inner-html.prop="props[item.name].action.tip_html(form[item.name], form)"></span>
          <el-button v-if="props[item.name].action.button_text" class="s2" size="mini" :loading="props[item.name].action.loading||false" type="primary" plain @click="actionHandler(form[item.name], props[item.name].action)">{{props[item.name].action.button_text}}</el-button>
        </div>

        <div class="t-action" v-if="props[item.name].model_action">
          <el-button class="s2" size="mini" :loading="props[item.name].model_action.loading||false" type="primary" plain @click="modelActionHandler(form[item.name], item.name, props[item.name].model_action)">{{props[item.name].model_action.button_text || 'Action'}}</el-button>
        </div>

        

      </el-form-item>
      </div>
    </el-form>

    <span slot="footer" class="dialog-footer">
      <el-button size="small" @click="close()">{{param.close_text || 'Close'}}</el-button>
      <el-button size="small" :disabled="loading" type="primary" @click="confrim()">
        {{param.confirm_text || 'Confirm'}}
      </el-button>
    </span>

  </el-dialog>


</template>
<script>
import vue from 'vue';
import { mapState, mapGetters } from 'vuex';
import store from '../../store/index';
import utils from '../../tea/utils';
import Base from '../../workflow/Base';
import {_} from 'tearust_utils';
import TeaIconButton from '../../components/TeaIconButton';

export default {
  components: {
    TeaIconButton,
  },
  data(){
    return {
      form: null,
      labels: null,
      rules: null,
      types: null,

      loading: true,
    };
  },
  computed: {
    // ...mapGetters([
    //   'layer1_account'
    // ]),
    ...mapState('modal', {
      visible:state => store.state.modal.common_form.visible,
      title: state => store.state.modal.common_form.title,
      param: state => store.state.modal.common_form.param,
    })
  },

  methods: {
    async openHandler(){
      this.wf = new Base();
      await this.wf.init();


      const open_cb = utils.mem.get('common_form--open_cb');
      if(open_cb){
        await open_cb(this.param);
      }

      this.initFormByTx();
      
      this.loading = false;
    },
    initFormByTx(){

      const form = {};
      const labels = {};
      const rules = {};
      const types = {};
      const props = {};

      _.each(this.param.props, (item, name)=>{
        const n = name;
        labels[n] = item.label || utils.form.nameToLabel(n);
        form[n] = _.has(item, 'default') ? _.get(item, 'default') : null;
        const type = item.type;
        types[n] = type;
        props[n] = item;
        props[n].name = n;

        if(!item.required){
          rules[n] = [];
        }
        else{
          rules[n] = [{required: true, message: `${labels[n]} is required.`}];
          if(item.rules){
            rules[n] = _.concat(item.rules, rules[n]);
          }
        }

        
        
      });

      this.form = form;
      this.labels = labels;
      this.types = types;
      this.rules = rules;
      this.props = props;

      
    },

    close(){
      this.$store.commit('modal/close', 'common_form');
      _.delay(()=>{
        this.loading = true;
      }, 500)
    },
    async confrim(){
      const ref = this.$refs['tx_form'];
      await ref.validate();
      const cb = utils.mem.get('common_form');
      if(cb){
        const form = this.form;
        await cb(form, ()=>{
          this.close();
        });
      }

    },
    async actionHandler(val, action){
      
      vue.set(action, 'loading', true);

      const cb = action.handler;
      const html = await cb(val);

      await utils.sleep(500);
      if(html){
        action.html = html;
      }
      
      vue.set(action, 'loading', false);
    },
    async modelActionHandler(val, name, action){
      vue.set(action, 'loading', true);

      const cb = action.handler;
      const value = await cb(val);

      if(value){
        if(action.ref){
          this.form[action.ref] = value;
        }
        else{
          this.form[name] = value;
        }
      }
      
      vue.set(action, 'loading', false);
    }
  }
}
</script>