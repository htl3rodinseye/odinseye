<template>
  <div id="ode-statistic">
    <!--<router-link to="/"><< /statistics</router-link>-->
    <div class="grid">
      <ode-box title="test">
        <ode-stat-circle :percentage="50"></ode-stat-circle>
      </ode-box>
    </div>
  </div>
</template>

<script>
  import OdeBox from "../ode-box";
  import OdeStatCircle from "../ode-stat-circle";
  import axios from "axios";

  export default {
    name: "ode-statistic",
    components: {OdeStatCircle, OdeBox},
    data() {
      return {
        tasks: {},
        groups: ['Alle'],
        selected: 'Alle'
      }
    },
    created() {
      this.getTasks()
    },
    methods: {

      getTasks: function () {
        console.log('Aufgaben/getTasks()')
        let url = 'http://caretaker.wurzer.cc:9040/exercises/'
        axios.get(url, {
          crossdomain: true,
          params: {
            exid: 0
          }
        })
          .then(data => {
            console.log('Aufgaben/getTasks(): Data loaded')
            console.log(data.json)
            /*this.tasks = JSON.parse(data.json);*/
            let sampleData = '{"0":{"id":"1","name":"Exercise 1","status":"0","group_name":"Basic","root":"home/odin","description":"Exercise 1 from the Basic category","attributes":null},"1":{"id":"2","name":"Exercise 2","status":"0","group_name":"Advanced","root":"home/odin","description":"Exercise 2 from the advanced category","attributes":null}}'
            console.log(sampleData)
            this.tasks = JSON.parse(sampleData)
            console.log(this.tasks)
            console.log('Aufgaben/getTasks(): Data processed')
            this.getGroups()
          })
          .catch(err => {
            console.error(err)
          })
      },
      getGroups: function () {
        console.log('Aufgaben/getGroups()')
        for (let i in this.tasks) {
          this.groups.push(this.tasks[i]['group_name'])
        }
        this.groups = Array.from(new Set(this.groups))
        console.log(this.groups)
      }
    }
  }
</script>

<style scoped>

</style>
