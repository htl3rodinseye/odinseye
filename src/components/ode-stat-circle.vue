<template>
  <vc-donut :sections="[{value: percentage, color: 'var(--color-secondary)'}]"
            background="var(--color-primary-accent-2)"
            :size="250"
            :thickness="24">
    <h1 v-if="percentage">{{percentage}}%</h1>
  </vc-donut>
</template>

<script>
  export default {
    name: "ode-stat-circle",
    props: {
      percentage: Number,
    }
  }
</script>

<style scoped>

</style>
