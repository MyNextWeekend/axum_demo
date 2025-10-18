import router from './router/index.js'
import { useUserStore } from './stores/user'
import { ElNotification } from 'element-plus'
import NProgress from 'nprogress' // progress bar
import 'nprogress/nprogress.css' // progress bar style
import { getToken } from '@/utils/auth.js' // get token from cookie

NProgress.configure({ showSpinner: false }) // NProgress Configuration

const whiteList = ['/login', '/auth-redirect'] // 白名单

router.beforeEach(async (to, from, next) => {
  // 开始进度条
  NProgress.start()

  // 设置页面的title
  document.title = to.meta.title || '高大上'

  const userStore = useUserStore()
  // 从 Cookies 获取用户的token
  const token = getToken()

  // 没有 token
  if (!token) {
    // 白名单直接放行
    if (whiteList.includes(to.path)) {
      return next()
    }
    // 跳转登录
    return next(`/login?redirect=${to.path}`)
  }

  // 已登录，访问登录页跳转首页
  if (to.path === '/login') return next({ path: '/' })

  // 已登录，处理动态路由
  if (!userStore.roles?.length) {
    try {
      // 获取用户信息
      await userStore.getPermission()
      // 基于角色生成可访菜单
      const accessRoutes = userStore.generateRoutes()

      // 动态添加路由
      accessRoutes.forEach((route) => router.addRoute(route))

      // 重新跳转一次当前路径
      return next({ ...to, replace: true })
    } catch (error) {
      console.log('路由守卫异常:', error)
      // 删除令牌并转到登录页面重新登录
      userStore.logout()
      ElNotification({
        message: err?.message || err || '异常，请重新登录',
        type: 'error',
      })
      return next({ path: '/login', query: { redirect: to.fullPath } })
    }
  }

  // 默认放行
  next()
})

router.afterEach(() => {
  // 关闭进度条
  NProgress.done()
})
