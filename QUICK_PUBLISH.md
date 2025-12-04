# 快速发布指南

## 🚀 一键发布到 GitHub

本项目已配置好所有文档和代码，可以直接发布到 GitHub。

### 仓库信息

- **仓库地址**: `git@github.com:lujing-jlu/art-design-pro-rs.git`
- **版本号**: `v1.0.0`
- **分支**: `main`

## 📋 发布步骤

### 方法 1: 使用自动化脚本（推荐）

```bash
# 运行初始版本发布脚本
./scripts/initial-release.sh
```

脚本会自动完成：
1. ✅ 检查 Git 状态
2. ✅ 添加所有文件
3. ✅ 创建初始提交
4. ✅ 配置远程仓库
5. ✅ 设置主分支
6. ✅ 强制推送代码
7. ✅ 创建版本标签
8. ✅ 推送标签

### 方法 2: 手动执行命令

```bash
# 1. 初始化 Git（如果还没有）
git init

# 2. 添加所有文件
git add .

# 3. 创建初始提交
git commit -m "feat: initial release v1.0.0"

# 4. 添加远程仓库
git remote add origin git@github.com:lujing-jlu/art-design-pro-rs.git

# 5. 设置主分支
git branch -M main

# 6. 强制推送（作为初始版本）
git push -u origin main --force

# 7. 创建标签
git tag -a v1.0.0 -m "Release v1.0.0"

# 8. 推送标签
git push origin v1.0.0 --force
```

## ✅ 发布后检查

### 1. 验证代码推送

访问仓库主页：
```
https://github.com/lujing-jlu/art-design-pro-rs
```

检查：
- ✅ README.md 正确显示
- ✅ 所有文件都已上传
- ✅ 目录结构正确

### 2. 创建 Release

1. 访问 Releases 页面：
   ```
   https://github.com/lujing-jlu/art-design-pro-rs/releases
   ```

2. 点击 "Create a new release"

3. 填写信息：
   - **Choose a tag**: 选择 `v1.0.0`
   - **Release title**: `Art Design Pro RS v1.0.0`
   - **Description**: 复制以下内容

```markdown
# Art Design Pro RS v1.0.0

🎉 首次发布！一个基于 Rust + Vue 3 的现代化后台管理系统。

## ✨ 核心功能

### 后端功能
- ✅ 用户认证系统（登录、注册、忘记密码）
- ✅ 用户中心（个人信息、密码修改、头像上传）
- ✅ 用户管理（CRUD、分页、搜索）
- ✅ 角色管理（CRUD、分页、搜索）
- ✅ 菜单管理（CRUD、树形结构）
- ✅ JWT 认证和权限控制
- ✅ 文件上传功能

### 前端功能
- ✅ 完整的管理界面
- ✅ 动态路由和菜单
- ✅ 权限控制
- ✅ 响应式设计

## 🔐 安全特性

- JWT 认证
- Argon2 密码哈希
- 密码强度验证
- 登录防暴力破解
- 文件类型和大小验证
- SQL 注入防护
- XSS 防护

## 🛠️ 技术栈

### 后端
- Rust 1.70+
- Actix Web 4.x
- SeaORM 0.12
- SQLite（支持 PostgreSQL/MySQL）
- JWT + Argon2

### 前端
- Vue 3.5+
- TypeScript 5.6+
- Vite 7.x
- Element Plus 2.x
- Pinia 3.x

## 📚 文档

- [快速开始](docs/快速开始.md)
- [用户中心功能说明](docs/用户中心功能说明.md)
- [前后端对接说明](docs/前后端对接说明.md)
- [部署指南](docs/DEPLOYMENT.md)
- [FAQ](docs/FAQ.md)
- [贡献指南](CONTRIBUTING.md)

## 🚀 快速开始

### 后端
```bash
cd backend
cargo run -p api
# 运行在 http://127.0.0.1:9123
```

### 前端
```bash
cd frontend
pnpm install
pnpm dev
# 运行在 http://localhost:4000
```

### 测试账号
所有密码均为 `123456`：
- Super（超级管理员）
- Admin（系统管理员）
- User（普通用户）

## 📊 项目统计

- 代码量：约 13,000+ 行
- 文件数：185+ 个
- API 接口：19 个
- 文档：16 篇

## 🙏 致谢

- [Art Design Pro](https://github.com/Daymychen/art-design-pro) - 前端模板
- [Actix Web](https://actix.rs/) - Rust Web 框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) - Rust ORM
- [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库

感谢所有开源项目和社区的支持！

---

**完整文档**: https://github.com/lujing-jlu/art-design-pro-rs
```

4. 点击 "Publish release"

### 3. 配置仓库设置

访问仓库设置：
```
https://github.com/lujing-jlu/art-design-pro-rs/settings
```

#### 基本信息
- **Description**: `A Modern Admin Dashboard System Built with Rust + Vue 3`
- **Website**: 你的演示网站（如果有）
- **Topics**: 添加以下标签
  ```
  rust
  vue3
  typescript
  actix-web
  admin-dashboard
  backend
  frontend
  full-stack
  seaorm
  element-plus
  admin-template
  management-system
  jwt-authentication
  ```

#### 功能设置
- ✅ Issues
- ✅ Discussions（可选）
- ✅ Wiki（可选）

## 🎨 可选：添加徽章

在 README.md 顶部已包含以下徽章：

```markdown
![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Vue](https://img.shields.io/badge/Vue-3.5+-brightgreen.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
```

可以添加更多徽章：

```markdown
![Build](https://github.com/lujing-jlu/art-design-pro-rs/workflows/Rust%20CI/badge.svg)
![GitHub stars](https://img.shields.io/github/stars/lujing-jlu/art-design-pro-rs)
![GitHub forks](https://img.shields.io/github/forks/lujing-jlu/art-design-pro-rs)
![GitHub issues](https://img.shields.io/github/issues/lujing-jlu/art-design-pro-rs)
```

## 📸 可选：添加截图

1. 创建 `screenshots` 目录
2. 添加项目截图
3. 在 README.md 中引用

## 🌐 可选：部署演示

可以使用以下平台部署在线演示：
- **Vercel**: 前端部署
- **Railway**: 全栈部署
- **Fly.io**: Rust 后端
- **Render**: 全栈部署

## 📢 宣传项目

发布后可以在以下平台宣传：
- Twitter/X
- Reddit (r/rust, r/vuejs)
- Hacker News
- 掘金
- 知乎
- V2EX

## ✅ 发布完成检查清单

- [ ] 代码已推送到 GitHub
- [ ] 标签已创建并推送
- [ ] Release 已创建
- [ ] README 显示正常
- [ ] 仓库描述已添加
- [ ] 主题标签已添加
- [ ] Issues 已启用
- [ ] CI/CD 运行正常（如果配置了）

## 🎉 完成！

恭喜！你的项目已成功发布到 GitHub！

### 访问链接

- **仓库主页**: https://github.com/lujing-jlu/art-design-pro-rs
- **Releases**: https://github.com/lujing-jlu/art-design-pro-rs/releases
- **Issues**: https://github.com/lujing-jlu/art-design-pro-rs/issues

### 下一步

1. 监控 Issues 和 PR
2. 收集用户反馈
3. 持续改进
4. 发布新版本

---

**祝项目获得更多 Star！⭐**
