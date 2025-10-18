import request from '@/utils/request'

export function EndpointAdd(data) {
  return request({
    url: '/endpoint/add',
    method: 'post',
    data: data,
  })
}

export function EndpointDelete(data) {
  return request({
    url: '/endpoint/delete',
    method: 'post',
    data: data,
  })
}

export function EndpointUpdate(data) {
  return request({
    url: '/endpoint/update',
    method: 'post',
    data: data,
  })
}

export function EndpointQuery(data) {
  return request({
    url: '/endpoint/query',
    method: 'post',
    data: data,
  })
}

export function EndpointStatistic(data) {
  return request({
    url: '/endpoint/statistic',
    method: 'post',
    data: data,
  })
}

export function EndpointInfo(data) {
  return request({
    url: '/endpoint/info',
    method: 'post',
    data: data,
  })
}
