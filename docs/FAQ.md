# 常见问题 (FAQ)

## 目录

- [安装和配置](#安装和配置)
- [开发相关](#开发相关)
- [部署相关](#部署相关)
- [功能使用](#功能使用)
- [错误排查](#错误排查)

## 安装和配置

### Q: 如何安装 Rust？

**A:** 使用 rustup 安装：

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 下载并运行 https://rustup.rs/
```

### Q: 如何安装 pnpm？

**A:** 使用 npm 安装：

```bash
npm install -g pnpm
```

### Q: 后端端口被占用怎么办？

**A:** 修改 `backend/api/src/main.rs` 中的端口号：

```rust
.bind(("127.0.0.1", 9123))?  // 改为其他端口，如 8080
```

同时修改前端代理配置 `frontend/.env.development`：

```
VITE_API_PROXY_URL=http://127.0.0.1:8080
```

### Q: 如何切换数据库？

**A:** 修改环境变量 `DATABASE_URL`：

```bash
# SQLite (默认)
DATABASE_URL=sqlite://data.db?mode=rwc

# PostgreSQL
DATABASE_URL=postgres://user:password@localhost/dbname

# MySQL
DATABASE_URL=mysql://user:password@localhost/dbname
```

同时需要修改 `backend/api/Cargo.toml` 中的 features。

## 开发相关

### Q: 如何启用热重载？

**A:** 后端使用 cargo-watch：

```bash
cargo install cargo-watch
cd backend
cargo watch -x 'run -p api'
```

前端自带热重载：

```bash
cd frontend
pnpm dev
```

### Q: 如何运行测试？

**A:** 

后端测试：
```bash
cd backend
cargo test
./test_api.sh
./test_user_center.sh
```

前端测试：
```bash
cd frontend
pnpm lint
pnpm build  # 包含类型检查
```

### Q: 如何添加新的 API 接口？

**A:** 按以下步骤：

1. 在 `backend/common/src/models.rs` 定义数据模型
2. 在 `backend/service/src/*.rs` 实现业务逻辑
3. 在 `backend/api/src/handlers/*.rs` 实现 HTTP 处理器
4. 在 `backend/api/src/main.rs` 注册路由
5. 在 `frontend/src/api/*.ts` 添加前端 API 调用
6. 在 `frontend/src/types/api/api.d.ts` 添加类型定义

### Q: 如何添加新的数据库字段？

**A:** 

1. 修改 `backend/entity/src/*.rs` 实体定义
2. 创建新的迁移文件或修改现有迁移
3. 运行迁移（重启后端会自动执行）
4. 更新相关的服务和 API

## 部署相关

### Q: 如何部署到生产环境？

**A:** 请参考 [部署指南](DEPLOYMENT.md)。

简要步骤：
1. 后端：`cargo build --release`
2. 前端：修改 `.env.production` 后 `pnpm build`
3. 配置 Nginx 或其他 Web 服务器
4. 使用 systemd 管理后端服务

### Q: 如何配置 HTTPS？

**A:** 使用 Nginx 反向代理：

```nginx
server {
    listen 443 ssl http2;
    server_name yourdomain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location /api/ {
        proxy_pass http://127.0.0.1:9123;
    }
    
    location / {
        root /var/www/frontend/dist;
        try_files $uri $uri/ /index.html;
    }
}
```

### Q: 如何备份数据？

**A:** 

SQLite：
```bash
cp backend/data.db backup/data.db.$(date +%Y%m%d)
```

PostgreSQL：
```bash
pg_dump dbname > backup.sql
```

MySQL：
```bash
mysqldump -u root -p dbname > backup.sql
```

## 功能使用

### Q: 忘记管理员密码怎么办？

**A:** 

方法 1：使用忘记密码功能（如果配置了邮箱）

方法 2：直接修改数据库：
```bash
cd backend
cargo run --bin hash_gen  # 生成新密码的哈希
# 然后在数据库中更新密码字段
```

方法 3：删除数据库重新初始化：
```bash
rm backend/data.db
cargo run -p api  # 会重新创建并初始化
```

### Q: 如何添加新用户？

**A:** 

1. 使用管理员账号登录
2. 进入"系统管理" -> "用户管理"
3. 点击"新增用户"
4. 填写信息并选择角色
5. 默认密码为 123456

### Q: 如何自定义菜单？

**A:** 

1. 使用超级管理员账号登录
2. 进入"系统管理" -> "菜单管理"
3. 可以添加、编辑、删除菜单
4. 设置菜单的角色权限

### Q: 如何修改密码强度要求？

**A:** 修改 `backend/service/src/auth.rs` 中的 `validate_password_strength` 函数。

### Q: 如何限制文件上传大小？

**A:** 

前端限制在 `frontend/src/views/system/user-center/index.vue`：
```typescript
if (file.size > 2 * 1024 * 1024) {  // 2MB
  ElMessage.error('图片大小不能超过 2MB')
  return
}
```

后端限制在 Actix Web 配置中。

## 错误排查

### Q: 后端启动失败，提示数据库连接错误

**A:** 

1. 检查 `DATABASE_URL` 配置是否正确
2. 确保数据库服务正在运行
3. 检查数据库权限
4. 查看详细错误日志

### Q: 前端无法连接后端

**A:** 

1. 确认后端服务已启动
2. 检查 `.env.development` 中的 `VITE_API_PROXY_URL`
3. 确认 `.env` 中 `VITE_ACCESS_MODE=backend`
4. 检查浏览器控制台的网络请求
5. 检查 CORS 配置

### Q: 登录后立即退出

**A:** 

可能原因：
1. JWT token 配置问题
2. 后端返回 401 错误
3. 前端 token 存储失败

解决方法：
1. 检查浏览器控制台错误
2. 检查后端日志
3. 清除浏览器缓存和 localStorage

### Q: 文件上传失败

**A:** 

1. 检查 `backend/uploads` 目录是否存在且有写权限
2. 检查文件大小是否超过限制
3. 检查文件类型是否允许
4. 查看后端日志

### Q: 编译错误：找不到某个 crate

**A:** 

```bash
cd backend
cargo clean
cargo update
cargo build
```

### Q: 前端构建失败

**A:** 

```bash
cd frontend
rm -rf node_modules pnpm-lock.yaml
pnpm install
pnpm build
```

### Q: 数据库迁移失败

**A:** 

1. 删除数据库文件重新初始化
2. 检查迁移脚本是否有语法错误
3. 查看详细错误信息

### Q: 性能问题

**A:** 

后端优化：
1. 使用 `--release` 模式编译
2. 调整数据库连接池大小
3. 添加适当的索引
4. 使用缓存

前端优化：
1. 启用生产构建
2. 使用 CDN 加速静态资源
3. 启用 Gzip 压缩
4. 优化图片大小

## 其他问题

### Q: 如何贡献代码？

**A:** 请查看 [贡献指南](../CONTRIBUTING.md)。

### Q: 如何报告 Bug？

**A:** 在 GitHub 上创建 Issue，使用 Bug 报告模板。

### Q: 如何请求新功能？

**A:** 在 GitHub 上创建 Issue，使用功能请求模板。

### Q: 项目使用什么许可证？

**A:** MIT License，详见 [LICENSE](../LICENSE)。

### Q: 可以用于商业项目吗？

**A:** 可以，MIT License 允许商业使用。

### Q: 如何获取技术支持？

**A:** 

1. 查看文档
2. 搜索已有的 Issues
3. 创建新的 Issue
4. 参与 GitHub Discussions

---

如果你的问题没有在这里找到答案，请：

1. 查看 [文档](.)
2. 搜索 [GitHub Issues](https://github.com/lujing-jlu/art-design-pro-rs/issues)
3. 创建新的 [Issue](https://github.com/lujing-jlu/art-design-pro-rs/issues/new)

我们会尽快回复！
