import request from '@/utils/request'

export function MenuAdd(data) {
  return request({
    url: '/menu/create',
    method: 'post',
    data: data,
  })
}

export function MenuDelete(data) {
  return request({
    url: '/menu/delete',
    method: 'post',
    data: data,
  })
}

export function MenuUpdate(data) {
  return request({
    url: '/menu/update',
    method: 'post',
    data: data,
  })
}

export function MenuQuery(data) {
  return request({
    url: '/menu/query',
    method: 'post',
    data: data,
  })
}

export function MenuInfo(data) {
  return request({
    url: '/menu/info',
    method: 'post',
    data: data,
  })
}
