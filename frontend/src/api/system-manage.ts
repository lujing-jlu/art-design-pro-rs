import request from '@/utils/http'
import { AppRouteRecord } from '@/types/router'

// 获取用户列表
export function fetchGetUserList(params: Api.SystemManage.UserSearchParams) {
  return request.get<Api.SystemManage.UserList>({
    url: '/api/user/list',
    params
  })
}

// 创建用户
export function fetchCreateUser(params: Api.SystemManage.UserCreateParams) {
  return request.post<Api.SystemManage.UserListItem>({
    url: '/api/user',
    params
  })
}

// 更新用户
export function fetchUpdateUser(params: Api.SystemManage.UserUpdateParams) {
  return request.put<Api.SystemManage.UserListItem>({
    url: '/api/user',
    params
  })
}

// 删除用户
export function fetchDeleteUser(id: number) {
  return request.del<void>({
    url: `/api/user/${id}`
  })
}

// 获取角色列表
export function fetchGetRoleList(params: Api.SystemManage.RoleSearchParams) {
  return request.get<Api.SystemManage.RoleList>({
    url: '/api/role/list',
    params
  })
}

// 创建角色
export function fetchCreateRole(params: Api.SystemManage.RoleCreateParams) {
  return request.post<Api.SystemManage.RoleListItem>({
    url: '/api/role',
    params
  })
}

// 更新角色
export function fetchUpdateRole(params: Api.SystemManage.RoleUpdateParams) {
  return request.put<Api.SystemManage.RoleListItem>({
    url: '/api/role',
    params
  })
}

// 删除角色
export function fetchDeleteRole(id: number) {
  return request.del<void>({
    url: `/api/role/${id}`
  })
}

// 获取菜单列表
export function fetchGetMenuList() {
  return request.get<AppRouteRecord[]>({
    url: '/api/menu/list'
  })
}

// 新建菜单
export function fetchCreateMenu(params: Api.SystemManage.MenuPayload) {
  return request.post<AppRouteRecord>({
    url: '/api/menu',
    params
  })
}

// 更新菜单
export function fetchUpdateMenu(params: Api.SystemManage.MenuPayload) {
  return request.put<AppRouteRecord>({
    url: '/api/menu',
    params
  })
}

// 删除菜单
export function fetchDeleteMenu(id: number) {
  return request.del<void>({
    url: `/api/menu/${id}`
  })
}
