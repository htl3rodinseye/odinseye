<template>
  <div class="ode-box">
    <h1 v-if="title">{{title}}</h1>
    <div class="wrapper" padd="10px">
      <slot></slot>
    </div>
  </div>
</template>

<script>
  export default {
    name: "ode-box",
    props: {
      title: String
    }
  }
</script>

<style scoped>



</style>
