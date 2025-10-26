import request from '@/utils/request'

export function login(data) {
  return request({
    url: '/user/login',
    method: 'post',
    data,
  })
}

export function permission() {
  return request({
    url: '/user/permission',
    method: 'post',
    data: {},
  })
}

export function logout(token) {
  return request({
    url: '/user/logout',
    method: 'post',
    headers: { token: token },
  })
}

export function UserAdd(data) {
  return request({
    url: '/user/add',
    method: 'post',
    data: data,
  })
}

export function UserDelete(data) {
  return request({
    url: '/user/delete',
    method: 'post',
    data: data,
  })
}

export function UserUpdate(data) {
  return request({
    url: '/user/update',
    method: 'post',
    data: data,
  })
}

export function UserQuery(data) {
  return request({
    url: '/user/query',
    method: 'post',
    data: data,
  })
}

export function UserInfo(data) {
  return request({
    url: '/user/info',
    method: 'post',
    data: data,
  })
}
