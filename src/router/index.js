import {createRouter, createWebHistory, createWebHashHistory} from 'vue-router';
import { defineAsyncComponent } from 'vue';

// 静态导入页面
import Home from '../views/Home.vue';
import Settings from '../views/Setting.vue';
import Notify from '../views/Notify.vue';


const router = createRouter({ 
  // history: createWebHashHistory(),  // hash 模式
  history: createWebHistory(),  // history 模式
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
      meta: {
        title: '主页',
      },
    },
    {
      path: '/settings',
      name: 'settings',
      component: Settings,
      meta: {
        title: '设置页',
      },
    },
    {
      path: '/notify',
      name: 'notify',
      component: Notify,
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