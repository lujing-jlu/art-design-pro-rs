# 更新日志

所有重要的项目变更都会记录在这个文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 新增
- 用户中心完整功能
  - 个人信息管理（姓名、昵称、邮箱、手机、性别、地址、个人简介）
  - 密码修改功能（验证旧密码、密码强度校验）
  - 头像上传功能（文件类型和大小验证）
- 完整的前后端对接
- 数据库字段扩展（real_name, address, bio）
- 三个新的 API 接口
  - `PUT /api/user/profile` - 更新个人信息
  - `PUT /api/user/password` - 修改密码
  - `POST /api/user/avatar` - 上传头像
- 测试脚本 `test_user_center.sh`
- 详细的功能文档

### 改进
- 扩展 UserInfo 数据结构
- 优化用户中心页面交互
- 增强密码安全性（强度验证、Argon2 哈希）
- 完善错误处理和用户提示

### 文档
- 新增《用户中心功能说明》
- 新增《快速开始指南》
- 新增《实现总结》
- 更新主 README
- 新增贡献指南
- 新增 GitHub Issue 和 PR 模板

## [1.0.0] - 2024-12-04

### 新增
- 基于 Rust + Actix Web 的后端实现
- 基于 Vue 3 + TypeScript 的前端实现
- 用户认证系统
  - 登录、注册、忘记密码
  - JWT token 认证
  - 登录防暴力破解
- 系统管理功能
  - 用户管理（CRUD、分页、搜索）
  - 角色管理（CRUD、分页、搜索）
  - 菜单管理（CRUD、树形结构）
- 动态路由和权限控制
- 数据库自动迁移
- 预置测试账号
- API 测试脚本

### 技术栈
- 后端：Rust, Actix Web, SeaORM, SQLite, JWT, Argon2
- 前端：Vue 3, TypeScript, Vite, Element Plus, Pinia
- 数据库：SQLite（支持 PostgreSQL/MySQL）

### 安全特性
- JWT 认证
- Argon2 密码哈希
- 登录防暴力破解
- 基于角色的权限控制
- SQL 注入防护
- XSS 防护

---

## 版本说明

### 版本号格式
- 主版本号：不兼容的 API 修改
- 次版本号：向下兼容的功能性新增
- 修订号：向下兼容的问题修正

### 变更类型
- `新增`：新功能
- `改进`：对现有功能的改进
- `修复`：Bug 修复
- `废弃`：即将移除的功能
- `移除`：已移除的功能
- `安全`：安全相关的修复
- `文档`：文档更新

[Unreleased]: https://github.com/lujing-jlu/art-design-pro-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/lujing-jlu/art-design-pro-rs/releases/tag/v1.0.0
