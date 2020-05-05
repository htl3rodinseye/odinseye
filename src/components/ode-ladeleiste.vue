<template>
  <div>
    <template v-if="title"></template>
    <div class="ladeleiste">
      <div class="row"
           :style="{
        'background-color': color,
        'width': percentage + '%'
      }">
        <span v-if="title">{{title}}:</span>
        <span v-if="percentage > 20">{{percentage}}%</span>
      </div>
    </div>
  </div>
</template>

<script>
  export default {
    name: "ode-ladeleiste",
    props: {
      percentage: Number,
      color: String,
      title: String
    }
  }
</script>

<style scoped>

</style>
