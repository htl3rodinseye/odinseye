<template>
  <div id="ode-overview" class="grid">
    <ode-box-route to="/tasks" title="Aufgaben">
      <div class="col">
        <i class="fas fa-glasses"/>
      </div>
    </ode-box-route>
    <ode-box-route title="Statistics" to="">

    </ode-box-route>
    <ode-box-route title="Information" to="/info">
      <i class="fas fa-question-circle"/>
    </ode-box-route>
  </div>
</template>

<script>
  import OdeBox from "../ode-box";
  import OdeBoxRoute from "../ode-box-route";

  export default {
    name: "ode-overview",
    components: {OdeBoxRoute, OdeBox}
  }
</script>

<style scoped>

  #ode-overview {
    grid-template-columns: 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr;
  }

  a {
    height: 100%;
  }

  /*#ode-overview a:last-child:nth-child(odd){
    grid-column: 2/6;
  }*/

  i {
    font-size: 5em;
  }

</style>
