<template>
  <div id="ode-Aufgabe" class="grid">
    <h1>{{task.name}}</h1>
    <span>Status: {{task.status}}</span>
  </div>
</template>

<script>
  import axios from "axios";

  export default {
    name: "ode-aufgabe",
    data() {
      return {}
    },
    created() {
      this.getTask()
    },
    methods: {
      getTasks: async function () {
        console.log('Aufgabe/getTasks()')
        let url = 'http://caretaker.wurzer.cc:9040/exercises/'
        try {
          console.log('Aufgaben/getTasks(): Data loaded')

          this.task = await fetch(url + '?exid=' + this.$router.params.id).then(response => {
            return response.json()
          })
          console.log(this.task)

          console.log('Aufgaben/getTasks(): Data processed')

          this.getGroups()
        } catch (err) {
          console.error(err)
        }
      }
    }
  }
</script>

<style scoped>

</style>
