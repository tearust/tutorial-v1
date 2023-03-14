<template>
<div class="tea-page">
  <h4>Welcome to Sample Actor testing page.</h4>

  <el-button type="primary" @click="send_request()">Click here to send request</el-button>
</div>
</template>
<script>
import {_} from 'tearust_utils';
import layer2 from '../layer2';
export default {
  data(){
    return {};
  },
  methods: {
    async send_request(){
      const _axios = layer2.base.getAxios();
      try{
        const rs = await _axios.post('/say-hello', {
          sample: true,
          address: '0x000000000000000000000000000000000000000f'
        });
        alert(JSON.stringify(rs));
      }catch(e){
        alert(e.toString());
      }
      
    }
  }
};
</script>