import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/reader/:scoreId?',
      name: 'score-reader',
      component: () => import('../components/ScoreReader/ScoreReader.vue'),
      props: true
    }
  ]
})

export default router
