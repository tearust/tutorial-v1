<template>
<el-table-column
  v-bind="{...$props, ...$attrs}" 
  v-on="$listeners"
  :render-header="renderHeader"
>
  <template slot-scope="scope">
    <slot :row="scope.row"></slot>
  </template>
  
</el-table-column>

</template>
<script>
export default {
  props: {
    tip: {
      type: String,
      default: '',
    },
    link: {
      type: String,
      default: '',
    }
  },
  data(){
    return {
      ext_props: {},
    };
  },
  mounted(){
    
  },
  methods: {
    renderHeader(h, {column}){
      // return column.label;
      if(!this.tip && !this.link){
        return column.label;
      }

      const args = [];
      if(this.tip){
        args.push('el-tooltip');
        args.push({
          props: {
            content: this.tip,
            placement: 'top',
            effect: 'light',
          }
        });
      }
      else{
        args.push('div');
        args.push({
          style: {
            'display': 'inline-block',
            'cursor': (this.link ? 'pointer' : 'default'),
          }
        })
      }
      
      args.push([
        h('span', {
          class: {
            'iconfont icon-questionmark': true
          },
          style: {
            'margin-left': '5px',
            'position': 'relative',
            'top': '0',
            'font-size': '11px',
          },
          on: {
            click: ()=>{
              if(this.link){
                window.open(this.link, "_blank");
              }
            }
          }
        })
      ])


      return [
        column.label,
        h.apply(this, args)
      ];
    }
  },
}
</script>