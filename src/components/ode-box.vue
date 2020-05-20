<template>
  <div class="ode-box">
    <h1 v-if="title">{{title}}</h1>
    <div :style="padding: padding">
      <slot></slot>
    </div>
  </div>
</template>

<script>
  export default {
    name: "ode-box",
    props: {
      title: String,
      padding: Number
    }
  }
</script>

<style scoped>



</style>
