<template>
  <nav>
    <!--<ode-nav-button route="/">
      <i class="fas fa-align-justify"/>
    </ode-nav-button>-->
    <ode-nav-button route="/">
      <i class="fas fa-home"/>
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
  </nav>
</template>

<script>
  import OdeNavButton from "./ode-nav-button";

  export default {
    name: "ode-nav-bar",
    components: {OdeNavButton}
  }
</script>

<style scoped>



</style>
