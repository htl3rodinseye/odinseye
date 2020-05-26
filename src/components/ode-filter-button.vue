<template>
  <div>
    <input type="checkbox" :id="group_name" :value="group_name" v-model="selectedGroups">
    <label :for="group_name">{{group_name}}</label>
  </div>
</template>

<script>
  export default {
    name: "ode-filter-button",
    props: {
      group_name: String,
      selectedGroups: Array
    }
  }
</script>

<style scoped>

</style>
