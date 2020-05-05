<template>
  <router-link :to="to" class="ode-box">
    <h1 v-if="title">{{title}}</h1>
    <div class="wrapper" padd="10px">
      <slot></slot>
    </div>
  </router-link>
</template>

<script>
    export default {
        name: "ode-box-route",
      props: {
        title: String,
        to: String
      }
    }
</script>

<style scoped>

</style>
