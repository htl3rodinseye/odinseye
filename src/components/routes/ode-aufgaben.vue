<template>
  <div id="ode-aufgaben">
    <!--<router-link to="/">back</router-link>-->
    <div class="grid">
      <ode-box v-for="task in tasks"
               :title="task.name">
        <h2>{{task.group_name}}</h2>
        <p>{{task.description}}</p>
      </ode-box>
    </div>
  </div>
</template>

<script>
  import OdeBox from "../ode-box";

  export default {
    name: "Aufgaben",
    components: {OdeBox},
    data () {
      return {
        loading: false,
        post: null,
        error: null
      }
    },
    created () {
      // fetch the data when the view is created and the data is
      // already being observed
      this.fetchData()
    },
    watch: {
      // call again the method if the route changes
      '$route': 'fetchData'
    },
    methods: {
      fetchData () {
        this.error = this.post = null
        this.loading = true
        // replace `getPost` with your data fetching util / API wrapper
        getPost(this.$route.params.id, (err, post) => {
          this.loading = false
          if (err) {
            this.error = err.toString()
          } else {
            this.post = post
          }
        })
      }
    }
  }
</script>

<style scoped>

  #ode-aufgaben > div {
    grid-template-columns: 1fr 1fr 1fr;
  }

</style>
