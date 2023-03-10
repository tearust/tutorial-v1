<template>
  <div id="app">
    <PageHeader />
    <section class="tea-section">
      <keep-alive>
        <router-view v-if="$route.meta.keepAlive"></router-view>
      </keep-alive>
      <router-view v-if="!$route.meta.keepAlive"></router-view>
      
    </section>
    
    <PageFooter />
  </div>
</template>

<script>
import PageHeader from './components/PageHeader';
import PageFooter from './components/PageFooter';


export default {
  components: {
    PageHeader,
    PageFooter,

  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
#app .tea-section{
  margin: 0 auto;
  width: 1080px;
  padding: 24px 0 40px;
}
</style>
