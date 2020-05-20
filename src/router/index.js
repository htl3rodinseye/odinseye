import Vue from 'vue'
import Router from 'vue-router'
import odeAufgaben from '../components/routes/ode-aufgaben';
import odeOverview from '../components/routes/ode-overview';
import odeInfo from '../components/routes/ode-info'
import odeStatistic from '../components/routes/ode-statistic'

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
      path: '/statistic',
      name: 'ode-statistic',
      component: odeStatistic
    },
    {
      path: '/info',
      name: 'ode-info',
      component: odeInfo
    }
  ]
})
