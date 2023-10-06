import {createRouter, createWebHistory, createWebHashHistory} from 'vue-router'
import { defineAsyncComponent } from 'vue'

const router = createRouter({ 
  // history: createWebHashHistory(),  // hash 模式
  history: createWebHistory(),  // history 模式
  routes: [
    {
      path: '/',
      name: 'home',
      component: defineAsyncComponent(() => import(/* @vite-ignore */`../views/Home.vue`)),
      meta: {
        title: '主页',
      },
    },
    {
      path: '/settings',
      name: 'settings',
      component: defineAsyncComponent(() => import(/* @vite-ignore */`../views/setting.vue`)),
      meta: {
        title: '设置页',
      },
    },
    {
      path: '/notify',
      name: 'notify',
      component: defineAsyncComponent(() => import(/* @vite-ignore */`../views/notify.vue`)),
      meta: {
        title: '通知页',
      }
    },
    {
      path: '/*',
      redirect: '/',
    },
  ]
})

// 全局路由守卫
router.beforeEach((to, from, next)=>{
  // console.log(to, from)
  if (to.meta.title) {
    document.title = `${to.meta.title}`;
  }
  next()
})

router.afterEach((to, from)=>{
  // console.log(to, from)
  console.log('afterEach')
})

export default router