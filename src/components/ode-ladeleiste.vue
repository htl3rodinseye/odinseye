<template>
  <div>
    <template v-if="title"></template>
    <div class="ladeleiste">
      <div class="row"
           :style="{
        'background-color': color,
        'width': percentage + '%'
      }">
        <template v-if="title && percentage > 20">
          <span>{{title}}:</span>
          <span>{{percentage}}%</span>
        </template>
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
