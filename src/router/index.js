import Vue from 'vue'
import Router from 'vue-router'
import odeAufgaben from '../components/routes/ode-aufgaben';
import odeOverview from '../components/routes/ode-overview';
import odeInfo from '../components/routes/ode-info'

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: '/',
      name: 'ode-overview',
      component: odeOverview
    },
    {
      path: '/tasks',
      name: 'ode-aufgaben',
      component: odeAufgaben
    },
    {
      path: '/info',
      name: 'ode-info',
      component: odeInfo
    }
  ]
})
