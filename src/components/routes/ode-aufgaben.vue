<template>
  <div id="ode-aufgaben">
    <!--<router-link to="/">back</router-link>-->
    <button :click="getTasks()">Update Data</button>
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
  import axios from 'axios'
  import OdeBox from "../ode-box";

  export default {
    name: "Aufgaben",
    components: {OdeBox},
    data() {
      return {
        tasks: {}
      }
    },
    watch: {
      // call again the method if the route changes
      '$route': 'fetchData'
    },
    methods: {
      getTasks() {
        let url = 'http://caretaker.wurzer.cc:9040/exercises/'
        axios.get(url, {
          crossdomain: true,
          params: {
            exid: 0
          }
        })
          .then(data => {
            this.tasks = data.json
          })
          .catch(err => {
            console.error(err)
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
