<template>
  <div id="app" class="theme-dark">
    <ode-nav-bar/>
    <router-view class="wrapper" padd="5vh"></router-view>
  </div>
</template>

<script>
  import OdeNavBar from "./components/ode-nav-bar";
  export default {
    name: 'App',
    components: {OdeNavBar}
  }
</script>

<style>

  @import './assets/css/main.css';
  #app {
    height: 100vh;
    width: 100%;
    background: var(--color-primary);

    display: grid;
    grid-template-columns: auto 1fr;
  }
  #app nav{
    height: 100%;
    grid-column: 0/1;

    background-color: var(--color-primary-accent-2);
  }
</style>
