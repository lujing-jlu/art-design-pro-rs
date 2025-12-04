#!/bin/bash

# Art Design Pro RS - 初始版本发布脚本
# 此脚本将项目作为初始版本强制推送到 GitHub

set -e

echo "🚀 Art Design Pro RS - 初始版本发布"
echo "===================================="
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 仓库地址
REPO_URL="git@github.com:lujing-jlu/art-design-pro-rs.git"
VERSION="v1.0.0"

echo -e "${BLUE}目标仓库: ${REPO_URL}${NC}"
echo -e "${BLUE}版本号: ${VERSION}${NC}"
echo ""

# 确认操作
echo -e "${YELLOW}⚠️  警告: 此操作将强制推送到远程仓库，会覆盖远程的所有内容！${NC}"
echo -e "${YELLOW}   请确保你了解此操作的后果。${NC}"
echo ""
read -p "是否继续？(yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo -e "${RED}❌ 操作已取消${NC}"
    exit 1
fi

echo ""
echo "📋 步骤 1: 检查 Git 状态"
echo "------------------------"

# 检查是否在 git 仓库中
if [ ! -d ".git" ]; then
    echo "初始化 Git 仓库..."
    git init
    echo -e "${GREEN}✓ Git 仓库已初始化${NC}"
else
    echo -e "${GREEN}✓ 已在 Git 仓库中${NC}"
fi

echo ""
echo "📋 步骤 2: 添加所有文件"
echo "------------------------"

# 添加所有文件
git add .
echo -e "${GREEN}✓ 所有文件已添加${NC}"

echo ""
echo "📋 步骤 3: 创建初始提交"
echo "------------------------"

# 创建提交
git commit -m "feat: initial release ${VERSION}

🎉 首次发布 Art Design Pro RS

## 核心功能
- ✅ 完整的 Rust 后端实现（Actix Web + SeaORM）
- ✅ 完整的 Vue 3 前端实现（TypeScript + Element Plus）
- ✅ 用户认证系统（登录、注册、忘记密码）
- ✅ 用户中心（个人信息、密码修改、头像上传）
- ✅ 系统管理（用户、角色、菜单 CRUD）
- ✅ 动态路由和权限控制
- ✅ JWT 认证和 Argon2 密码哈希

## 技术栈
- 后端: Rust + Actix Web + SeaORM + SQLite
- 前端: Vue 3 + TypeScript + Vite + Element Plus
- 数据库: SQLite（支持 PostgreSQL/MySQL）

## 文档
- 📚 16 篇详细文档
- 🌐 中英双语 README
- 🚀 快速开始指南
- 📖 部署指南
- ❓ FAQ

## 安全特性
- 🔐 JWT 认证
- 🔒 Argon2 密码哈希
- 🛡️ 登录防暴力破解
- ✅ 权限控制

## 测试
- ✅ 后端测试脚本
- ✅ 用户中心测试脚本
- ✅ CI/CD 配置

预置测试账号（密码均为 123456）：
- Super（超级管理员）
- Admin（系统管理员）
- User（普通用户）
" || echo -e "${YELLOW}⚠️  提交可能已存在，继续...${NC}"

echo -e "${GREEN}✓ 提交已创建${NC}"

echo ""
echo "📋 步骤 4: 配置远程仓库"
echo "------------------------"

# 检查远程仓库是否已存在
if git remote | grep -q "^origin$"; then
    echo "移除现有的 origin..."
    git remote remove origin
fi

# 添加远程仓库
git remote add origin "$REPO_URL"
echo -e "${GREEN}✓ 远程仓库已配置${NC}"

echo ""
echo "📋 步骤 5: 设置主分支"
echo "------------------------"

# 设置主分支名称
git branch -M main
echo -e "${GREEN}✓ 主分支已设置为 main${NC}"

echo ""
echo "📋 步骤 6: 强制推送到远程仓库"
echo "------------------------"

echo -e "${YELLOW}正在推送到 ${REPO_URL}...${NC}"

# 强制推送
if git push -u origin main --force; then
    echo -e "${GREEN}✓ 代码已成功推送到远程仓库${NC}"
else
    echo -e "${RED}❌ 推送失败，请检查网络连接和仓库权限${NC}"
    exit 1
fi

echo ""
echo "📋 步骤 7: 创建版本标签"
echo "------------------------"

# 创建标签
git tag -a "$VERSION" -m "Release ${VERSION}

首次稳定版本发布，包含完整功能：

## 新增功能
- 用户认证系统
- 用户、角色、菜单管理
- 用户中心功能
- 完整的文档
- 生产就绪的部署配置

## 技术栈
- 后端: Rust + Actix Web + SeaORM + SQLite
- 前端: Vue 3 + TypeScript + Vite + Element Plus

## 安全特性
- JWT 认证
- Argon2 密码哈希
- 登录防暴力破解
- 基于角色的权限控制

## 文档
- 快速开始指南
- 用户中心功能说明
- 部署指南
- FAQ
- 贡献指南
"

echo -e "${GREEN}✓ 标签 ${VERSION} 已创建${NC}"

echo ""
echo "📋 步骤 8: 推送标签"
echo "------------------------"

# 推送标签
if git push origin "$VERSION" --force; then
    echo -e "${GREEN}✓ 标签已成功推送${NC}"
else
    echo -e "${RED}❌ 标签推送失败${NC}"
    exit 1
fi

echo ""
echo "✅ 初始版本发布完成！"
echo "===================="
echo ""
echo -e "${GREEN}🎉 恭喜！项目已成功发布到 GitHub！${NC}"
echo ""
echo "📊 发布信息:"
echo "  - 仓库地址: ${REPO_URL}"
echo "  - 版本号: ${VERSION}"
echo "  - 分支: main"
echo ""
echo "🔗 访问链接:"
echo "  - 仓库主页: https://github.com/lujing-jlu/art-design-pro-rs"
echo "  - Releases: https://github.com/lujing-jlu/art-design-pro-rs/releases"
echo "  - Issues: https://github.com/lujing-jlu/art-design-pro-rs/issues"
echo ""
echo "📝 下一步操作:"
echo "  1. 访问 GitHub 仓库查看代码"
echo "  2. 在 Releases 页面创建正式发布"
echo "  3. 配置仓库设置（描述、主题标签等）"
echo "  4. 添加项目截图（可选）"
echo "  5. 宣传你的项目！"
echo ""
echo "📚 更多信息:"
echo "  - 查看 PUBLISH_TO_GITHUB.md 了解详细步骤"
echo "  - 查看 PROJECT_SUMMARY.md 了解项目概况"
echo "  - 查看 docs/快速开始.md 了解如何使用"
echo ""
echo -e "${BLUE}🌟 如果项目对你有帮助，请给一个 Star！${NC}"
echo ""
