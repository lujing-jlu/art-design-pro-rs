# 部署指南

本文档介绍如何将 Art Design Pro RS 部署到生产环境。

## 目录

- [系统要求](#系统要求)
- [后端部署](#后端部署)
- [前端部署](#前端部署)
- [数据库配置](#数据库配置)
- [反向代理配置](#反向代理配置)
- [Docker 部署](#docker-部署)
- [常见问题](#常见问题)

## 系统要求

### 最低配置
- CPU: 1 核
- 内存: 512MB
- 磁盘: 1GB

### 推荐配置
- CPU: 2 核+
- 内存: 2GB+
- 磁盘: 10GB+

### 软件要求
- 操作系统: Linux (Ubuntu 20.04+, CentOS 7+) / macOS / Windows
- Rust: 1.70+
- Node.js: 20.19.0+ (仅构建时需要)
- 数据库: SQLite 3 / PostgreSQL 12+ / MySQL 8+

## 后端部署

### 1. 编译

```bash
cd backend

# 生产构建
cargo build --release

# 编译后的二进制文件位于
# backend/target/release/api
```

### 2. 配置环境变量

创建 `.env` 文件：

```bash
# 数据库连接
DATABASE_URL=sqlite://data.db?mode=rwc
# 或使用 PostgreSQL
# DATABASE_URL=postgres://user:password@localhost/dbname
# 或使用 MySQL
# DATABASE_URL=mysql://user:password@localhost/dbname

# JWT 密钥（生产环境请使用强密钥）
JWT_SECRET=your-super-secret-key-change-this-in-production

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=9123

# 日志级别
RUST_LOG=info
```

### 3. 运行

```bash
# 直接运行
./target/release/api

# 或使用 systemd（推荐）
sudo cp art-design-pro-rs.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable art-design-pro-rs
sudo systemctl start art-design-pro-rs
```

### 4. Systemd 服务配置

创建 `/etc/systemd/system/art-design-pro-rs.service`：

```ini
[Unit]
Description=Art Design Pro RS Backend
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/art-design-pro-rs/backend
Environment="DATABASE_URL=sqlite:///opt/art-design-pro-rs/backend/data.db?mode=rwc"
Environment="RUST_LOG=info"
ExecStart=/opt/art-design-pro-rs/backend/target/release/api
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

## 前端部署

### 1. 构建

```bash
cd frontend

# 安装依赖
pnpm install

# 配置生产环境 API 地址
# 编辑 .env.production
VITE_API_URL=https://api.yourdomain.com

# 构建
pnpm build

# 构建产物位于 frontend/dist
```

### 2. 部署到静态服务器

#### Nginx

```nginx
server {
    listen 80;
    server_name yourdomain.com;

    root /var/www/art-design-pro-rs/frontend/dist;
    index index.html;

    # Gzip 压缩
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # 静态资源缓存
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

#### Apache

```apache
<VirtualHost *:80>
    ServerName yourdomain.com
    DocumentRoot /var/www/art-design-pro-rs/frontend/dist

    <Directory /var/www/art-design-pro-rs/frontend/dist>
        Options -Indexes +FollowSymLinks
        AllowOverride All
        Require all granted

        # SPA 路由支持
        RewriteEngine On
        RewriteBase /
        RewriteRule ^index\.html$ - [L]
        RewriteCond %{REQUEST_FILENAME} !-f
        RewriteCond %{REQUEST_FILENAME} !-d
        RewriteRule . /index.html [L]
    </Directory>

    # 静态资源缓存
    <FilesMatch "\.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$">
        Header set Cache-Control "max-age=31536000, public, immutable"
    </FilesMatch>
</VirtualHost>
```

## 数据库配置

### SQLite（默认）

```bash
# 数据库文件会自动创建
DATABASE_URL=sqlite://data.db?mode=rwc
```

### PostgreSQL

```bash
# 1. 创建数据库
createdb art_design_pro

# 2. 配置连接
DATABASE_URL=postgres://username:password@localhost/art_design_pro

# 3. 修改 Cargo.toml
# 将 sqlx-sqlite 改为 sqlx-postgres
```

### MySQL

```bash
# 1. 创建数据库
mysql -u root -p
CREATE DATABASE art_design_pro CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

# 2. 配置连接
DATABASE_URL=mysql://username:password@localhost/art_design_pro

# 3. 修改 Cargo.toml
# 将 sqlx-sqlite 改为 sqlx-mysql
```

## 反向代理配置

### Nginx 完整配置

```nginx
# 后端 API
upstream backend {
    server 127.0.0.1:9123;
}

# 前端
server {
    listen 80;
    server_name yourdomain.com;

    # 重定向到 HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    # SSL 证书
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    # SSL 配置
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # 前端静态文件
    root /var/www/art-design-pro-rs/frontend/dist;
    index index.html;

    # API 代理
    location /api/ {
        proxy_pass http://backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket 支持（如果需要）
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }

    # 上传文件
    location /uploads/ {
        proxy_pass http://backend;
    }

    # 前端路由
    location / {
        try_files $uri $uri/ /index.html;
    }

    # 静态资源缓存
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## Docker 部署

### Dockerfile（后端）

```dockerfile
# backend/Dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

RUN cargo build --release -p api

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api /app/api
COPY --from=builder /app/migration /app/migration

ENV DATABASE_URL=sqlite://data.db?mode=rwc
ENV RUST_LOG=info

EXPOSE 9123

CMD ["./api"]
```

### Dockerfile（前端）

```dockerfile
# frontend/Dockerfile
FROM node:20-alpine as builder

WORKDIR /app
COPY package.json pnpm-lock.yaml ./

RUN npm install -g pnpm && pnpm install

COPY . .
RUN pnpm build

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

### docker-compose.yml

```yaml
version: '3.8'

services:
  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    ports:
      - "9123:9123"
    volumes:
      - ./backend/data:/app/data
      - ./backend/uploads:/app/uploads
    environment:
      - DATABASE_URL=sqlite:///app/data/data.db?mode=rwc
      - RUST_LOG=info
    restart: unless-stopped

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    ports:
      - "80:80"
    depends_on:
      - backend
    restart: unless-stopped
```

### 使用 Docker Compose 部署

```bash
# 构建并启动
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止
docker-compose down

# 重启
docker-compose restart
```

## 性能优化

### 后端优化

1. **启用 Release 模式**
   ```bash
   cargo build --release
   ```

2. **数据库连接池**
   - 已在代码中配置，默认最大连接数为 10

3. **日志级别**
   ```bash
   # 生产环境使用 info 或 warn
   RUST_LOG=info
   ```

### 前端优化

1. **代码分割**
   - 已配置，Vite 自动处理

2. **资源压缩**
   - 构建时自动压缩

3. **CDN 加速**
   - 将静态资源上传到 CDN

## 监控和日志

### 后端日志

```bash
# 查看实时日志
journalctl -u art-design-pro-rs -f

# 查看最近的日志
journalctl -u art-design-pro-rs -n 100
```

### 前端日志

```bash
# Nginx 访问日志
tail -f /var/log/nginx/access.log

# Nginx 错误日志
tail -f /var/log/nginx/error.log
```

## 备份

### 数据库备份

```bash
# SQLite
cp backend/data.db backend/data.db.backup

# PostgreSQL
pg_dump art_design_pro > backup.sql

# MySQL
mysqldump -u root -p art_design_pro > backup.sql
```

### 上传文件备份

```bash
tar -czf uploads-backup.tar.gz backend/uploads/
```

## 常见问题

### 1. 端口被占用

```bash
# 查看端口占用
lsof -i :9123

# 修改端口
# 编辑 backend/api/src/main.rs
.bind(("127.0.0.1", 9123))?  // 改为其他端口
```

### 2. 数据库连接失败

- 检查 DATABASE_URL 配置
- 确保数据库服务正在运行
- 检查数据库权限

### 3. 前端无法连接后端

- 检查 CORS 配置
- 确认 API 地址正确
- 检查防火墙设置

### 4. 文件上传失败

- 检查 uploads 目录权限
- 确认磁盘空间充足
- 检查文件大小限制

## 安全建议

1. **使用 HTTPS**
   - 配置 SSL 证书
   - 强制 HTTPS 重定向

2. **更改默认密码**
   - 修改预置账号密码
   - 使用强密码

3. **限制访问**
   - 配置防火墙
   - 使用 IP 白名单

4. **定期更新**
   - 更新依赖包
   - 应用安全补丁

5. **备份数据**
   - 定期备份数据库
   - 备份上传文件

## 更新部署

```bash
# 1. 拉取最新代码
git pull

# 2. 重新构建后端
cd backend
cargo build --release

# 3. 重新构建前端
cd ../frontend
pnpm install
pnpm build

# 4. 重启服务
sudo systemctl restart art-design-pro-rs
sudo systemctl reload nginx
```

---

如有问题，请查看 [常见问题](FAQ.md) 或提交 [Issue](https://github.com/lujing-jlu/art-design-pro-rs/issues)。
