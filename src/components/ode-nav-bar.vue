<template>
  <nav id="ode-nav-bar">
    <div>
      <!--<ode-nav-button route="/">
        <i class="fas fa-align-justify"/>
      </ode-nav-button>-->
      <ode-nav-button route="/" style="width: 10px">
        <i><img src="static/logo.png" style="width: 100%; height: auto"></i>
      </ode-nav-button>
      <div class="trennstrich"/>
      <ode-nav-button route="/tasks">
        <i class="fas fa-glasses"/>
      </ode-nav-button>
      <!--<ode-nav-button route="/statistic">
        <i class="fas fa-chart-pie"/>
      </ode-nav-button>-->
      <ode-nav-button route="/info">
        <i class="fas fa-question-circle"></i>
      </ode-nav-button>
    </div>
    <label class="switch" style="margin-top: 20vh">
      <input type="checkbox" id="theme-switcher" v-on:change="changeTheme()">
      <div>
        <span></span>
      </div>
    </label>
  </nav>
</template>

<script>
  import OdeNavButton from "./ode-nav-button";

  export default {
    name: "ode-nav-bar",
    components: {OdeNavButton},
    methods: {
      changeTheme: function () {
        let app = document.getElementById("app").classList;
        if (app.contains('theme-dark')) {
          app.remove('theme-dark')
          app.add('theme-light')
        } else if (app.contains('theme-light')) {
          app.remove('theme-light')
          app.add('theme-dark')
        }
      }
    }
  }
</script>

<style scoped>


</style>
