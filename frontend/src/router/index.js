import { createRouter, createWebHistory } from 'vue-router'

import Layout from '@/layout/index.vue'

// 公开的路由
export const publicRoutes = [
  {
    path: '/login',
    component: () => import('@/views/login/index.vue'),
  },
  {
    path: '/',
    component: Layout,
    redirect: '/dashboard',
    children: [
      {
        path: '/dashboard',
        component: () => import('@/views/dashboard/index.vue'),
        meta: { title: '首页', icon: 'dashboard' },
      },
    ],
  },
  {
    path: '/401',
    component: () => import('@/views/errorPage/401.vue'),
  },
  {
    path: '/404',
    component: () => import('@/views/errorPage/404.vue'),
  },
]

// 私有的路由
export const privateRoutes = [
  {
    path: '/news',
    component: Layout,
    redirect: '/news/dashboard',
    meta: { title: '实用工具', icon: 'computer' },
    children: [
      {
        path: '/news/dashboard',
        component: () => import('@/views/dashboard/index.vue'),
        meta: { title: '工具001', icon: 'lighting', roles: ['admin'] },
      },
      {
        path: '/news/user',
        component: () => import('@/views/user/index.vue'),
        meta: { title: '工具002', icon: 'game' },
      },
    ],
  },
  {
    path: '/summer',
    component: Layout,
    redirect: '/summer/summer',
    meta: { title: '设置', icon: 'setting' },
    children: [
      {
        path: '/summer/summer',
        component: () => import('@/views/summer/index.vue'),
        meta: { title: '用户信息', icon: 'user' },
      },
      {
        path: '/summer/user',
        component: () => import('@/views/user/index.vue'),
        meta: { title: '个人中心', icon: 'document' },
      },
      {
        path: '/summer/menu',
        component: () => import('@/views/menu/index.vue'),
        meta: { title: '路由配置', icon: 'document' },
      },
    ],
  },
  { path: '/:pathMatch(.*)*', redirect: '/404' },
]

// 初始化路由只有公开路由
const create = () =>
  createRouter({
    history: createWebHistory(),
    routes: publicRoutes,
  })

const router = create()

// 退出登陆 之后 重置路由
export function resetRouter() {
  const newRouter = create()
  router.matcher = newRouter.matcher
}

export default router
