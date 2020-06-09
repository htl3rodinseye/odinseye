<template>
  <div id="ode-overview">
    <div class="grid">
      <ode-box-route to="/tasks" title="Aufgaben">
        <div class="col">
          <i class="fas fa-glasses"/>
        </div>
        <p>Hier kannst du alle Aufgaben einsehen</p>
      </ode-box-route>
      <!--<ode-box-route title="Statistics" to="">
        <div class="col">
          <i class="fas fa-chart-pie"/>
        </div>
        <p>Hier kannst du deinen Fortschritt aller Aufgaben zu jeder Kategorie einsehen</p>
      </ode-box-route>-->
      <ode-box-route title="Information" to="/info">
        <div class="col">
          <i class="fas fa-question-circle"/>
        </div>
        <p>Hier sind zusätzliche Informationen zum Projekt und ein Installationsguide</p>
      </ode-box-route>
      <!--<ode-box class="span-2">
        <div style="font-size: 5em">
          <a class="button" href="./static/odins_eye" download><i class="fas fa-download"/> Download</a>
        </div>
      </ode-box>-->
    </div>
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

  #ode-overview > div.grid {
    grid-template-columns: 1fr 1fr;
    grid-template-rows: auto;
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
