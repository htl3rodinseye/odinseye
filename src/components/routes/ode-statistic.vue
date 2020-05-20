<template>
  <div id="ode-statistic">
    <ode-box title="test">
      <ode-stat-circle :percentage="50"></ode-stat-circle>
    </ode-box>
  </div>
</template>

<script>
  import OdeBox from "../ode-box";
  import OdeStatCircle from "../ode-stat-circle";

  export default {
    name: "ode-statistic",
    components: {OdeStatCircle, OdeBox}
  }
</script>

<style scoped>

</style>
