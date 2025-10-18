import request from '@/utils/request'

export function ParameterAdd(data) {
  return request({
    url: '/parameter/add',
    method: 'post',
    data: data,
  })
}

export function ParameterDelete(data) {
  return request({
    url: '/parameter/delete',
    method: 'post',
    data: data,
  })
}

export function ParameterUpdate(data) {
  return request({
    url: '/parameter/update',
    method: 'post',
    data: data,
  })
}

export function ParameterQuery(data) {
  return request({
    url: '/parameter/query',
    method: 'post',
    data: data,
  })
}

export function ParameterStatistic(data) {
  return request({
    url: '/parameter/statistic',
    method: 'post',
    data: data,
  })
}

export function ParameterInfo(data) {
  return request({
    url: '/parameter/info',
    method: 'post',
    data: data,
  })
}
