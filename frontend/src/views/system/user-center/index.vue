<!-- 个人中心页面 -->
<template>
  <div class="w-full h-full p-0 bg-transparent border-none shadow-none">
    <div class="relative flex-b mt-2.5 max-md:block max-md:mt-1">
      <div class="w-112 mr-5 max-md:w-full max-md:mr-0">
        <div class="art-card-sm relative p-9 pb-6 overflow-hidden text-center">
          <img class="absolute top-0 left-0 w-full h-50 object-cover" src="@imgs/user/bg.webp" />
          <div class="relative z-10 mt-30 mx-auto">
            <img
              class="w-20 h-20 mx-auto object-cover border-2 border-white rounded-full cursor-pointer hover:opacity-80"
              :src="userInfo.avatar || '@imgs/user/avatar.webp'"
              @click="handleAvatarClick"
            />
            <input
              ref="avatarInput"
              type="file"
              accept="image/*"
              style="display: none"
              @change="handleAvatarChange"
            />
          </div>
          <h2 class="mt-5 text-xl font-normal">{{ userInfo.userName }}</h2>
          <p class="mt-5 text-sm">专注于用户体验跟视觉设计</p>

          <div class="w-75 mx-auto mt-7.5 text-left">
            <div class="mt-2.5">
              <ArtSvgIcon icon="ri:mail-line" class="text-g-700" />
              <span class="ml-2 text-sm">{{ userInfo.email || '未设置邮箱' }}</span>
            </div>
            <div class="mt-2.5">
              <ArtSvgIcon icon="ri:user-3-line" class="text-g-700" />
              <span class="ml-2 text-sm">{{ userInfo.realName || '未设置姓名' }}</span>
            </div>
            <div class="mt-2.5">
              <ArtSvgIcon icon="ri:map-pin-line" class="text-g-700" />
              <span class="ml-2 text-sm">{{ userInfo.address || '未设置地址' }}</span>
            </div>
            <div class="mt-2.5">
              <ArtSvgIcon icon="ri:phone-line" class="text-g-700" />
              <span class="ml-2 text-sm">{{ userInfo.phone || '未设置手机' }}</span>
            </div>
          </div>

          <div class="mt-10">
            <h3 class="text-sm font-medium">标签</h3>
            <div class="flex flex-wrap justify-center mt-3.5">
              <div
                v-for="item in lableList"
                :key="item"
                class="py-1 px-1.5 mr-2.5 mb-2.5 text-xs border border-g-300 rounded"
              >
                {{ item }}
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="flex-1 overflow-hidden max-md:w-full max-md:mt-3.5">
        <div class="art-card-sm">
          <h1 class="p-4 text-xl font-normal border-b border-g-300">基本设置</h1>

          <ElForm
            :model="form"
            class="box-border p-5 [&>.el-row_.el-form-item]:w-[calc(50%-10px)] [&>.el-row_.el-input]:w-full [&>.el-row_.el-select]:w-full"
            ref="ruleFormRef"
            :rules="rules"
            label-width="86px"
            label-position="top"
          >
            <ElRow>
              <ElFormItem label="姓名" prop="realName">
                <ElInput v-model="form.realName" :disabled="!isEdit" />
              </ElFormItem>
              <ElFormItem label="性别" prop="sex" class="ml-5">
                <ElSelect v-model="form.sex" placeholder="Select" :disabled="!isEdit">
                  <ElOption
                    v-for="item in options"
                    :key="item.value"
                    :label="item.label"
                    :value="item.value"
                  />
                </ElSelect>
              </ElFormItem>
            </ElRow>

            <ElRow>
              <ElFormItem label="昵称" prop="nikeName">
                <ElInput v-model="form.nikeName" :disabled="!isEdit" />
              </ElFormItem>
              <ElFormItem label="邮箱" prop="email" class="ml-5">
                <ElInput v-model="form.email" :disabled="!isEdit" />
              </ElFormItem>
            </ElRow>

            <ElRow>
              <ElFormItem label="手机" prop="mobile">
                <ElInput v-model="form.mobile" :disabled="!isEdit" />
              </ElFormItem>
              <ElFormItem label="地址" prop="address" class="ml-5">
                <ElInput v-model="form.address" :disabled="!isEdit" />
              </ElFormItem>
            </ElRow>

            <ElFormItem label="个人介绍" prop="des" class="h-32">
              <ElInput type="textarea" :rows="4" v-model="form.des" :disabled="!isEdit" />
            </ElFormItem>

            <div class="flex-c justify-end [&_.el-button]:!w-27.5">
              <ElButton type="primary" class="w-22.5" v-ripple @click="edit">
                {{ isEdit ? '保存' : '编辑' }}
              </ElButton>
            </div>
          </ElForm>
        </div>

        <div class="art-card-sm my-5">
          <h1 class="p-4 text-xl font-normal border-b border-g-300">更改密码</h1>

          <ElForm :model="pwdForm" class="box-border p-5" label-width="86px" label-position="top">
            <ElFormItem label="当前密码" prop="password">
              <ElInput
                v-model="pwdForm.password"
                type="password"
                :disabled="!isEditPwd"
                show-password
              />
            </ElFormItem>

            <ElFormItem label="新密码" prop="newPassword">
              <ElInput
                v-model="pwdForm.newPassword"
                type="password"
                :disabled="!isEditPwd"
                show-password
              />
            </ElFormItem>

            <ElFormItem label="确认新密码" prop="confirmPassword">
              <ElInput
                v-model="pwdForm.confirmPassword"
                type="password"
                :disabled="!isEditPwd"
                show-password
              />
            </ElFormItem>

            <div class="flex-c justify-end [&_.el-button]:!w-27.5">
              <ElButton type="primary" class="w-22.5" v-ripple @click="editPwd">
                {{ isEditPwd ? '保存' : '编辑' }}
              </ElButton>
            </div>
          </ElForm>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { useUserStore } from '@/store/modules/user'
  import type { FormInstance, FormRules } from 'element-plus'
  import { ElMessage } from 'element-plus'
  import { fetchUpdateProfile, fetchChangePassword, fetchUploadAvatar } from '@/api/auth'

  defineOptions({ name: 'UserCenter' })

  const userStore = useUserStore()
  const userInfo = computed(() => userStore.getUserInfo)

  const isEdit = ref(false)
  const isEditPwd = ref(false)
  const date = ref('')
  const ruleFormRef = ref<FormInstance>()
  const avatarInput = ref<HTMLInputElement>()

  /**
   * 用户信息表单
   */
  const form = reactive({
    realName: '',
    nikeName: '',
    email: '',
    mobile: '',
    address: '',
    sex: '1',
    des: ''
  })

  // 初始化表单数据
  watchEffect(() => {
    if (userInfo.value) {
      form.realName = userInfo.value.realName || ''
      form.nikeName = userInfo.value.nickName || userInfo.value.userName || ''
      form.email = userInfo.value.email || ''
      form.mobile = userInfo.value.phone || ''
      form.address = userInfo.value.address || ''
      form.sex = userInfo.value.gender === '女' ? '2' : '1'
      form.des = userInfo.value.bio || ''
    }
  })

  /**
   * 密码修改表单
   */
  const pwdForm = reactive({
    password: '',
    newPassword: '',
    confirmPassword: ''
  })

  /**
   * 表单验证规则
   */
  const rules = reactive<FormRules>({
    realName: [
      { required: true, message: '请输入姓名', trigger: 'blur' },
      { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' }
    ],
    nikeName: [
      { required: true, message: '请输入昵称', trigger: 'blur' },
      { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' }
    ],
    email: [{ required: true, message: '请输入邮箱', trigger: 'blur' }],
    mobile: [{ required: true, message: '请输入手机号码', trigger: 'blur' }],
    address: [{ required: true, message: '请输入地址', trigger: 'blur' }],
    sex: [{ required: true, message: '请选择性别', trigger: 'blur' }]
  })

  /**
   * 性别选项
   */
  const options = [
    { value: '1', label: '男' },
    { value: '2', label: '女' }
  ]

  /**
   * 用户标签列表
   */
  const lableList: Array<string> = ['专注设计', '很有想法', '辣~', '大长腿', '川妹子', '海纳百川']

  onMounted(() => {
    getDate()
  })

  /**
   * 根据当前时间获取问候语
   */
  const getDate = () => {
    const h = new Date().getHours()

    if (h >= 6 && h < 9) date.value = '早上好'
    else if (h >= 9 && h < 11) date.value = '上午好'
    else if (h >= 11 && h < 13) date.value = '中午好'
    else if (h >= 13 && h < 18) date.value = '下午好'
    else if (h >= 18 && h < 24) date.value = '晚上好'
    else date.value = '很晚了，早点睡'
  }

  /**
   * 点击头像触发文件选择
   */
  const handleAvatarClick = () => {
    avatarInput.value?.click()
  }

  /**
   * 处理头像文件选择
   */
  const handleAvatarChange = async (event: Event) => {
    const target = event.target as HTMLInputElement
    const file = target.files?.[0]
    if (!file) return

    // 验证文件类型
    if (!file.type.startsWith('image/')) {
      ElMessage.error('请选择图片文件')
      return
    }

    // 验证文件大小（限制为 2MB）
    if (file.size > 2 * 1024 * 1024) {
      ElMessage.error('图片大小不能超过 2MB')
      return
    }

    try {
      const updatedInfo = await fetchUploadAvatar(file)
      userStore.setUserInfo(updatedInfo)
      ElMessage.success('头像上传成功')
    } catch (error) {
      console.error('头像上传失败:', error)
      ElMessage.error('头像上传失败')
    } finally {
      // 清空 input，允许重复选择同一文件
      target.value = ''
    }
  }

  /**
   * 切换用户信息编辑状态
   */
  const edit = async () => {
    if (isEdit.value) {
      // 保存
      try {
        await ruleFormRef.value?.validate()
        const params: Api.Auth.UpdateProfileParams = {
          realName: form.realName,
          nickName: form.nikeName,
          email: form.email,
          phone: form.mobile,
          gender: form.sex === '2' ? '女' : '男',
          address: form.address,
          bio: form.des
        }
        const updatedInfo = await fetchUpdateProfile(params)
        // 更新 store 中的用户信息
        userStore.setUserInfo(updatedInfo)
        ElMessage.success('更新成功')
        isEdit.value = false
      } catch (error) {
        console.error('更新失败:', error)
      }
    } else {
      isEdit.value = true
    }
  }

  /**
   * 切换密码编辑状态
   */
  const editPwd = async () => {
    if (isEditPwd.value) {
      // 保存
      if (!pwdForm.password || !pwdForm.newPassword || !pwdForm.confirmPassword) {
        ElMessage.error('请填写完整的密码信息')
        return
      }
      if (pwdForm.newPassword !== pwdForm.confirmPassword) {
        ElMessage.error('两次输入的新密码不一致')
        return
      }
      try {
        await fetchChangePassword({
          oldPassword: pwdForm.password,
          newPassword: pwdForm.newPassword
        })
        ElMessage.success('密码修改成功，请重新登录')
        // 清空密码表单
        pwdForm.password = ''
        pwdForm.newPassword = ''
        pwdForm.confirmPassword = ''
        isEditPwd.value = false
        // 可选：自动登出
        setTimeout(() => {
          userStore.logout()
        }, 1500)
      } catch (error) {
        console.error('密码修改失败:', error)
      }
    } else {
      isEditPwd.value = true
    }
  }
</script>
