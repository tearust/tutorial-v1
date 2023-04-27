<template>
<div class="tea-page">
  <h4>Welcome to Sample Actor testing page.</h4>

  <el-button type="primary" @click="send_request()">Click here to send request</el-button>
</div>
</template>
<script>
import {_, axios} from 'tearust_utils';
export default {
  data(){
    return {};
  },
  methods: {
    get_env(key) {
      const x_key = 'VUE_APP_' + _.toUpper(key);
      return _.get(process.env, x_key, null);
    },
    async send_request(){
      const _axios = axios.create({
        baseURL: this.get_env('LAYER2_URL'),
      });
      const rs = await _axios.post('/say-hello', {
        actor: 'someone.sample',
        address: '0x000000000000000000000000000000000000000f'
      });
      alert(rs.data);
    }
  }
};
</script>