import { defineStore } from 'pinia'
import { publicRoutes, privateRoutes, resetRouter } from '@/router/index.js'
import { getToken, setToken, removeToken } from '@/utils/auth.js'
import { login, permission, logout } from '@/apis/user.js'

/**
 * 判断当前用户是否有权限访问该路由
 * @param roles
 * @param route
 */
function hasPermission(roles, route) {
  if (route.meta && route.meta?.roles) {
    return roles.some((role) => route.meta.roles?.includes(role))
  } else {
    return false
  }
}

/**
 * 过滤私有路由表，返回符合用户角色权限的路由表
 * @param routes asyncRoutes
 * @param roles
 */
export function filterAsyncRoutes(routes, roles) {
  const res = []

  routes.forEach((route) => {
    const tmp = { ...route }
    if (hasPermission(roles, tmp)) {
      if (tmp.children) {
        tmp.children = filterAsyncRoutes(tmp.children, roles)
      }
      res.push(tmp)
    }
  })
  return res
}

/**
 * 深度提取路由信息，只保留 path/meta/children
 * @param {Array} routes 路由数组
 * @returns {Array} 纯数据路由
 */
function extractRouteData(routes = []) {
  return routes.map((route) => {
    const { path, meta, children } = route
    return {
      path,
      meta,
      children: children ? extractRouteData(children) : [],
    }
  })
}

const useUserStore = defineStore('user', {
  state: () => ({
    name: '',
    token: getToken() || '',
    avatar: '',
    introduction: '',
    roles: [],

    routes: [],
  }),
  actions: {
    async login(userInfo) {
      const res = await login(userInfo)
      this.token = res.data.token
      setToken(this.token)
    },

    async getPermission() {
      const res = await permission()
      const { roles, name, avatar, introduction } = res.data
      if (!roles || roles.length <= 0) {
        reject('返回角色信息不能为空')
      }
      this.roles = roles
      this.name = name
      this.avatar = avatar
      this.introduction = introduction
    },

    async logout() {
      await logout()
      this.name = ''
      this.token = null
      this.roles = []
      this.routes = []
      removeToken() // 移除 Cookies 中的 token
      resetRouter() // 重置路由
    },

    generateRoutes() {
      let accessedRoutes
      if (this.roles.includes('admin')) {
        accessedRoutes = privateRoutes || []
      } else if (this.roles.length === 0) {
        accessedRoutes = []
      } else {
        accessedRoutes = filterAsyncRoutes(privateRoutes, this.roles)
      }
      // 去掉路由中的多余信息，只保留 path/meta/children
      this.routes = extractRouteData(publicRoutes.concat(accessedRoutes))
      return accessedRoutes
    },
  },
})

export { useUserStore }
