<template>
  <router-link class="row ode-nav-button" v-bind:to="route">
    <slot></slot>
  </router-link>
</template>

<script>
  export default {
    name: "ode-nav-button",
    props: [
      'route'
    ],
    methods: {}
  }
</script>

<style scoped>

</style>
