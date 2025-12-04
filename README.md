# Art Design Pro RS

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Vue](https://img.shields.io/badge/Vue-3.5+-brightgreen.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

一个基于 Rust + Vue 3 的现代化后台管理系统

[在线演示](#) | [快速开始](docs/快速开始.md) | [功能文档](docs/) | [贡献指南](CONTRIBUTING.md)

</div>

---

## 📖 项目简介

Art Design Pro RS 是一个全栈后台管理系统，采用 Rust 构建高性能后端，Vue 3 + TypeScript 构建现代化前端。项目提供了完整的用户管理、角色权限、菜单管理等企业级功能。

**特点**：
- 🚀 **高性能**：Rust 后端，极致性能和内存安全
- 🎨 **现代化**：Vue 3 + TypeScript + Vite，开发体验极佳
- 🔐 **安全可靠**：JWT 认证、Argon2 密码哈希、防暴力破解
- 📦 **开箱即用**：完整的 CRUD、权限控制、动态路由
- 🛠️ **易于扩展**：清晰的架构，模块化设计

## 后端实现（Rust）
- 框架与技术栈：Actix Web、SeaORM、SQLite、JWT（jsonwebtoken）、Argon2、Serde、tokio。
- Workspace 结构：`api`(HTTP 服务) / `common`(错误、JWT、密码哈希等) / `service`(业务逻辑) / `entity`(SeaORM 实体) / `migration`(自动迁移与种子)。
- 功能：
  - 认证：登录/注册/忘记密码、用户信息获取
  - 用户中心：个人信息管理、密码修改、头像上传
  - 系统管理：用户列表 CRUD、角色列表 CRUD、菜单列表 CRUD
  - 统一响应 `{ code, msg, data }`，401 触发前端自动登出
- 数据库与种子：默认 SQLite `backend/data.db`；迁移自动创建表并写入 3 个账号（Super/Admin/User，密码均为 123456，对应角色 R_SUPER/R_ADMIN/R_USER）。
- 测试脚本：
  - `backend/test_api.sh` - 验证登录、用户/角色/菜单接口
  - `backend/test_user_center.sh` - 验证用户中心功能（需 `jq`）

### 启动后端
```bash
cd backend
# 可选：export DATABASE_URL="sqlite://data.db?mode=rwc"  # 默认即为此值
cargo run -p api             # 启动于 http://127.0.0.1:9123
```

## 前端对接改造（基于上游开源前端）
- 访问模式切换为后端模式：`.env` 中 `VITE_ACCESS_MODE=backend`，开发代理指向 `http://127.0.0.1:9123`（见 `.env.development`）。
- 接口接入：
  - 认证：登录/注册/忘记密码页
  - 用户中心：个人信息编辑、密码修改、头像上传
  - 系统管理：用户/角色/菜单 CRUD
  - 动态菜单加载和权限控制
- 路由与菜单：后端返回的菜单结构与前端路由保持一致，RouteRegistry 动态注册并驱动侧边栏渲染。
- 数据约定：统一响应 `{ code, msg, data }`，401 时前端自动登出；更多对接细节见 `docs/前后端对接说明.md` 和 `docs/用户中心功能说明.md`。

### 启动前端（用于验证后端）
```bash
cd frontend
pnpm install
pnpm dev                      # http://localhost:4000
```
- 生产构建时请把 `.env.production` 的 `VITE_API_URL` 改为后端实际地址后执行 `pnpm build`。

## 主要接口
- 认证：`POST /api/auth/login`、`POST /api/auth/register`、`POST /api/auth/forget-password`、`GET /api/user/info`
- 用户中心：`PUT /api/user/profile`、`PUT /api/user/password`、`POST /api/user/avatar`
- 用户管理：`GET /api/user/list`、`POST /api/user`、`PUT /api/user`、`DELETE /api/user/{id}`
- 角色管理：`GET /api/role/list`、`POST /api/role`、`PUT /api/role`、`DELETE /api/role/{id}`
- 菜单管理：`GET /api/v3/system/menus/simple`、`GET /api/menu/list`、`POST /api/menu`、`PUT /api/menu`、`DELETE /api/menu/{id}`

### 预置账号（密码 123456）
| 用户名 | 角色代码 | 说明 |
| ------ | -------- | ---- |
| Super  | R_SUPER  | 超级管理员 |
| Admin  | R_ADMIN  | 系统管理员 |
| User   | R_USER   | 普通用户 |

## 部署提示
- 后端：`cargo run -p api --release`，通过 `DATABASE_URL` 切换数据库（SQLite/PostgreSQL/MySQL 均可，需匹配 SeaORM feature）。
- 前端：只需配置后端地址（`VITE_API_URL`），构建出的 `frontend/dist` 可交由任意静态服务器托管。

## 📸 项目截图

> 待添加：登录页面、控制台、用户管理、角色管理、菜单管理、用户中心等截图

## 🗺️ 路线图

- [x] 基础认证系统
- [x] 用户管理
- [x] 角色管理
- [x] 菜单管理
- [x] 用户中心
- [ ] 部门管理
- [ ] 操作日志
- [ ] 数据字典
- [ ] 文件管理
- [ ] 系统监控
- [ ] 在线用户
- [ ] 定时任务

## 🤝 贡献

欢迎贡献代码！请查看 [贡献指南](CONTRIBUTING.md)。

### 贡献者

感谢所有为这个项目做出贡献的人！

<!-- 这里可以添加贡献者列表 -->

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

前端部分基于 [Art Design Pro](https://github.com/Daymychen/art-design-pro)，遵循其开源许可。

## 🙏 致谢

- [Art Design Pro](https://github.com/Daymychen/art-design-pro) - 前端模板
- [Actix Web](https://actix.rs/) - Rust Web 框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) - Rust ORM
- [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库

## 📞 联系方式

- 提交 Issue: [GitHub Issues](https://github.com/lujing-jlu/art-design-pro-rs/issues)
- 讨论: [GitHub Discussions](https://github.com/lujing-jlu/art-design-pro-rs/discussions)

## ⭐ Star History

如果这个项目对你有帮助，请给一个 Star ⭐

[![Star History Chart](https://api.star-history.com/svg?repos=lujing-jlu/art-design-pro-rs&type=Date)](https://star-history.com/#lujing-jlu/art-design-pro-rs&Date)
