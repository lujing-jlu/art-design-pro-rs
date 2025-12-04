import request from '@/utils/http'

/**
 * 登录
 * @param params 登录参数
 * @returns 登录响应
 */
export function fetchLogin(params: Api.Auth.LoginParams) {
  return request.post<Api.Auth.LoginResponse>({
    url: '/api/auth/login',
    params
    // showSuccessMessage: true // 显示成功消息
    // showErrorMessage: false // 不显示错误消息
  })
}

/**
 * 获取用户信息
 * @returns 用户信息
 */
export function fetchGetUserInfo() {
  return request.get<Api.Auth.UserInfo>({
    url: '/api/user/info'
    // 自定义请求头
    // headers: {
    //   'X-Custom-Header': 'your-custom-value'
    // }
  })
}

/**
 * 注册
 * @param params 注册参数
 */
export function fetchRegister(params: Api.Auth.RegisterParams) {
  return request.post<Api.Auth.RegisterResponse>({
    url: '/api/auth/register',
    params
  })
}

/**
 * 重置密码
 * @param params 重置密码参数
 */
export function fetchResetPassword(params: Api.Auth.ResetPasswordParams) {
  return request.post<Api.Auth.ResetPasswordResponse>({
    url: '/api/auth/forget-password',
    params
  })
}

/**
 * 更新个人信息
 * @param params 个人信息参数
 */
export function fetchUpdateProfile(params: Api.Auth.UpdateProfileParams) {
  return request.put<Api.Auth.UserInfo>({
    url: '/api/user/profile',
    params
  })
}

/**
 * 修改密码
 * @param params 修改密码参数
 */
export function fetchChangePassword(params: Api.Auth.ChangePasswordParams) {
  return request.put<void>({
    url: '/api/user/password',
    params
  })
}

/**
 * 上传头像
 * @param file 头像文件
 */
export function fetchUploadAvatar(file: File) {
  const formData = new FormData()
  formData.append('file', file)
  return request.post<Api.Auth.UserInfo>({
    url: '/api/user/avatar',
    params: formData,
    headers: {
      'Content-Type': 'multipart/form-data'
    }
  })
}
