# 发布到 GitHub 完整指南

## 🎯 发布前最终检查

在执行发布命令之前，请确认：

- [x] 所有代码已完成
- [x] 所有文档已编写
- [x] 测试全部通过
- [x] 代码已格式化
- [x] 无编译警告
- [x] 已阅读 RELEASE_CHECKLIST.md

## 📋 发布步骤

### 步骤 1: 运行发布准备脚本

```bash
# 赋予执行权限
chmod +x scripts/prepare-release.sh

# 运行脚本（会自动清理、检查、生成发布信息）
./scripts/prepare-release.sh
```

脚本会自动：
- ✅ 清理临时文件（target, node_modules, data.db 等）
- ✅ 检查代码质量
- ✅ 验证文档完整性
- ✅ 扫描敏感信息
- ✅ 生成 RELEASE_NOTES.md

### 步骤 2: 创建 GitHub 仓库

1. 登录 GitHub: https://github.com
2. 点击右上角 "+" -> "New repository"
3. 填写信息：
   - **Repository name**: `art-design-pro-rs`
   - **Description**: `A Modern Admin Dashboard System Built with Rust + Vue 3`
   - **Public** 或 **Private**（根据需要选择）
   - ❌ **不要**勾选 "Initialize this repository with:"
     - ❌ Add a README file
     - ❌ Add .gitignore
     - ❌ Choose a license
4. 点击 "Create repository"

### 步骤 3: 初始化本地 Git 仓库（如果还没有）

```bash
# 检查是否已经是 git 仓库
git status

# 如果不是，初始化
git init

# 添加所有文件
git add .

# 首次提交
git commit -m "feat: initial release v1.0.0

- Complete backend implementation with Rust + Actix Web
- Complete frontend implementation with Vue 3 + TypeScript
- User authentication and authorization
- User, role, and menu management
- User center with profile management
- Comprehensive documentation
- CI/CD configuration
"
```

### 步骤 4: 连接远程仓库并推送

```bash
# 添加远程仓库（替换 yourusername 为你的 GitHub 用户名）
git remote add origin https://github.com/lujing-jlu/art-design-pro-rs.git

# 设置主分支名称
git branch -M main

# 推送代码
git push -u origin main
```

### 步骤 5: 创建版本标签

```bash
# 创建标签
git tag -a v1.0.0 -m "Release v1.0.0

First stable release with complete features:
- User authentication system
- User, role, and menu management
- User center functionality
- Comprehensive documentation
- Production-ready deployment
"

# 推送标签
git push origin v1.0.0

# 或推送所有标签
git push origin --tags
```

### 步骤 6: 在 GitHub 上创建 Release

1. 访问你的仓库页面
2. 点击右侧的 "Releases"
3. 点击 "Create a new release"
4. 填写信息：
   - **Choose a tag**: 选择 `v1.0.0`
   - **Release title**: `Art Design Pro RS v1.0.0`
   - **Description**: 复制 RELEASE_NOTES.md 的内容
5. 点击 "Publish release"

### 步骤 7: 配置仓库设置

#### 7.1 基本设置

1. 进入仓库 Settings
2. 在 "About" 部分：
   - 添加描述：`A Modern Admin Dashboard System Built with Rust + Vue 3`
   - 添加网站（如果有）
   - 添加主题标签（Topics）：
     ```
     rust, vue3, typescript, actix-web, admin-dashboard,
     backend, frontend, full-stack, seaorm, element-plus,
     admin-template, management-system, jwt-authentication
     ```

#### 7.2 功能设置

在 Settings -> General 中：
- ✅ 启用 Issues
- ✅ 启用 Discussions（可选）
- ✅ 启用 Wiki（可选）
- ✅ 启用 Projects（可选）

#### 7.3 分支保护（可选）

在 Settings -> Branches 中：
- 添加分支保护规则 for `main`
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging

### 步骤 8: 验证发布

1. **检查 README 显示**
   - 访问仓库主页
   - 确认 README.md 正确显示
   - 检查所有链接是否有效

2. **检查 CI/CD**
   - 进入 Actions 标签
   - 确认 workflows 运行正常
   - 查看构建状态

3. **检查 Release**
   - 进入 Releases 页面
   - 确认 v1.0.0 显示正确
   - 下载源码压缩包测试

## 🎨 可选：添加项目徽章

在 README.md 顶部添加徽章（已包含在 README 中）：

```markdown
![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Vue](https://img.shields.io/badge/Vue-3.5+-brightgreen.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)
![Build](https://github.com/lujing-jlu/art-design-pro-rs/workflows/Rust%20CI/badge.svg)
```

## 📸 可选：添加项目截图

1. 创建 `screenshots` 目录
2. 添加截图：
   - login.png（登录页面）
   - dashboard.png（控制台）
   - user-management.png（用户管理）
   - role-management.png（角色管理）
   - menu-management.png（菜单管理）
   - user-center.png（用户中心）

3. 在 README.md 中引用：
```markdown
## 📸 项目截图

### 登录页面
![登录页面](screenshots/login.png)

### 控制台
![控制台](screenshots/dashboard.png)

### 用户管理
![用户管理](screenshots/user-management.png)
```

## 🌐 可选：部署在线演示

### 使用 GitHub Pages（前端）

1. 构建前端：
```bash
cd frontend
pnpm build
```

2. 在 Settings -> Pages 中：
   - Source: Deploy from a branch
   - Branch: gh-pages
   - Folder: / (root)

3. 推送到 gh-pages 分支：
```bash
cd dist
git init
git add .
git commit -m "Deploy to GitHub Pages"
git push -f https://github.com/lujing-jlu/art-design-pro-rs.git main:gh-pages
```

### 使用其他平台

- **Vercel**: 适合前端部署
- **Railway**: 适合全栈部署
- **Fly.io**: 适合 Rust 后端
- **Render**: 适合全栈部署

## 📢 宣传项目

### 1. 社交媒体

- Twitter/X
- Reddit (r/rust, r/vuejs)
- Hacker News
- 掘金
- 知乎
- V2EX

### 2. 技术社区

- Rust 中文社区
- Vue.js 中文社区
- GitHub Trending
- Awesome Lists

### 3. 博客文章

写一篇介绍文章：
- 项目背景和动机
- 技术选型和架构
- 开发过程和经验
- 未来计划

## 🔄 后续维护

### 定期任务

1. **监控 Issues**
   - 及时回复用户问题
   - 标记和分类 Issues
   - 关闭已解决的 Issues

2. **审查 Pull Requests**
   - 代码审查
   - 测试验证
   - 合并或提供反馈

3. **更新文档**
   - 根据反馈更新文档
   - 添加新功能文档
   - 修正错误

4. **发布新版本**
   - 遵循语义化版本
   - 更新 CHANGELOG.md
   - 创建新的 Release

### 版本发布流程

```bash
# 1. 更新版本号
# 编辑 backend/Cargo.toml 和 frontend/package.json

# 2. 更新 CHANGELOG.md
# 添加新版本的变更记录

# 3. 提交变更
git add .
git commit -m "chore: bump version to v1.1.0"

# 4. 创建标签
git tag -a v1.1.0 -m "Release v1.1.0"

# 5. 推送
git push origin main --tags

# 6. 在 GitHub 上创建 Release
```

## ✅ 发布完成检查清单

- [ ] 代码已推送到 GitHub
- [ ] 标签已创建
- [ ] Release 已发布
- [ ] README 显示正常
- [ ] CI/CD 运行正常
- [ ] 所有链接有效
- [ ] 仓库设置完成
- [ ] 主题标签已添加
- [ ] 项目描述已添加

## 🎉 恭喜！

你的项目已成功发布到 GitHub！

### 下一步

1. 分享你的项目
2. 收集用户反馈
3. 持续改进
4. 发布新版本

### 获取帮助

如有问题：
- 查看 [FAQ](docs/FAQ.md)
- 搜索 [GitHub Issues](https://github.com/lujing-jlu/art-design-pro-rs/issues)
- 创建新的 Issue

---

**祝你的项目获得更多 Star！⭐**

**Happy Coding! 🚀**
