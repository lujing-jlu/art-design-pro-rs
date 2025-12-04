#!/bin/bash

# Art Design Pro RS - 发布准备脚本
# 此脚本帮助清理项目并准备发布到 GitHub

set -e

echo "🚀 Art Design Pro RS - 发布准备"
echo "================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查是否在项目根目录
if [ ! -f "README.md" ] || [ ! -d "backend" ] || [ ! -d "frontend" ]; then
    echo -e "${RED}❌ 错误: 请在项目根目录运行此脚本${NC}"
    exit 1
fi

echo "📋 步骤 1: 清理临时文件"
echo "------------------------"

# 清理后端
echo "清理后端 target 目录..."
if [ -d "backend/target" ]; then
    rm -rf backend/target
    echo -e "${GREEN}✓ 已删除 backend/target${NC}"
fi

# 清理数据库文件
echo "清理测试数据库..."
if [ -f "backend/data.db" ]; then
    rm -f backend/data.db backend/data.db-shm backend/data.db-wal
    echo -e "${GREEN}✓ 已删除测试数据库${NC}"
fi

# 清理上传文件
echo "清理测试上传文件..."
if [ -d "backend/uploads" ]; then
    rm -rf backend/uploads/*
    echo -e "${GREEN}✓ 已清理 uploads 目录${NC}"
fi

# 清理前端
echo "清理前端 node_modules..."
if [ -d "frontend/node_modules" ]; then
    rm -rf frontend/node_modules
    echo -e "${GREEN}✓ 已删除 frontend/node_modules${NC}"
fi

echo "清理前端 dist..."
if [ -d "frontend/dist" ]; then
    rm -rf frontend/dist
    echo -e "${GREEN}✓ 已删除 frontend/dist${NC}"
fi

# 清理系统文件
echo "清理系统文件..."
find . -name ".DS_Store" -type f -delete 2>/dev/null || true
find . -name "Thumbs.db" -type f -delete 2>/dev/null || true
echo -e "${GREEN}✓ 已清理系统文件${NC}"

echo ""
echo "📋 步骤 2: 检查代码质量"
echo "------------------------"

# 检查后端
echo "检查后端代码..."
cd backend

if ! cargo check --all-features 2>&1 | grep -q "Finished"; then
    echo -e "${RED}❌ 后端代码检查失败${NC}"
    exit 1
fi
echo -e "${GREEN}✓ 后端代码检查通过${NC}"

if ! cargo fmt -- --check 2>&1; then
    echo -e "${YELLOW}⚠️  后端代码格式需要调整，正在格式化...${NC}"
    cargo fmt
    echo -e "${GREEN}✓ 后端代码已格式化${NC}"
fi

cd ..

# 检查前端
echo "检查前端代码..."
cd frontend

if [ ! -d "node_modules" ]; then
    echo "安装前端依赖..."
    pnpm install
fi

if ! pnpm lint 2>&1 | grep -q "error"; then
    echo -e "${GREEN}✓ 前端代码检查通过${NC}"
else
    echo -e "${YELLOW}⚠️  前端代码有 lint 错误，尝试自动修复...${NC}"
    pnpm lint --fix || true
fi

cd ..

echo ""
echo "📋 步骤 3: 验证文档完整性"
echo "------------------------"

# 检查必需文档
REQUIRED_DOCS=(
    "README.md"
    "README_EN.md"
    "LICENSE"
    "CONTRIBUTING.md"
    "CHANGELOG.md"
    ".gitignore"
    "docs/快速开始.md"
    "docs/用户中心功能说明.md"
    "docs/前后端对接说明.md"
    "docs/实现总结.md"
    "docs/DEPLOYMENT.md"
)

MISSING_DOCS=()

for doc in "${REQUIRED_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        MISSING_DOCS+=("$doc")
    fi
done

if [ ${#MISSING_DOCS[@]} -eq 0 ]; then
    echo -e "${GREEN}✓ 所有必需文档都存在${NC}"
else
    echo -e "${RED}❌ 缺少以下文档:${NC}"
    for doc in "${MISSING_DOCS[@]}"; do
        echo "  - $doc"
    done
    exit 1
fi

echo ""
echo "📋 步骤 4: 检查敏感信息"
echo "------------------------"

# 检查是否有敏感信息
SENSITIVE_PATTERNS=(
    "password.*=.*[^123456]"
    "api[_-]?key"
    "secret[_-]?key.*=.*[^your-super-secret]"
    "token.*=.*[a-zA-Z0-9]{20,}"
)

echo "扫描敏感信息..."
FOUND_SENSITIVE=false

for pattern in "${SENSITIVE_PATTERNS[@]}"; do
    if grep -r -i -E "$pattern" --exclude-dir={node_modules,target,.git,dist} . 2>/dev/null; then
        FOUND_SENSITIVE=true
    fi
done

if [ "$FOUND_SENSITIVE" = true ]; then
    echo -e "${YELLOW}⚠️  发现可能的敏感信息，请检查上述内容${NC}"
    echo -e "${YELLOW}   如果是测试数据，可以忽略${NC}"
else
    echo -e "${GREEN}✓ 未发现明显的敏感信息${NC}"
fi

echo ""
echo "📋 步骤 5: 生成发布信息"
echo "------------------------"

# 获取版本号
VERSION=$(grep -m 1 "version" backend/Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
echo "当前版本: v$VERSION"

# 统计代码行数
echo ""
echo "代码统计:"
echo "  后端 Rust 代码:"
find backend -name "*.rs" -not -path "*/target/*" | xargs wc -l | tail -1
echo "  前端 TypeScript/Vue 代码:"
find frontend/src -name "*.ts" -o -name "*.vue" | xargs wc -l | tail -1
echo "  文档:"
find docs -name "*.md" | xargs wc -l | tail -1

echo ""
echo "📋 步骤 6: 创建发布检查清单"
echo "------------------------"

cat > RELEASE_NOTES.md << EOF
# Release v$VERSION

## 发布日期
$(date +%Y-%m-%d)

## 新增功能
- 用户中心完整功能（个人信息、密码修改、头像上传）
- 完整的前后端对接
- 详细的功能文档

## 改进
- 扩展用户信息字段
- 优化用户体验
- 增强安全性

## 技术栈
- 后端: Rust + Actix Web + SeaORM + SQLite
- 前端: Vue 3 + TypeScript + Vite + Element Plus

## 安装
请参考 [快速开始指南](docs/快速开始.md)

## 测试账号
- Super / 123456 (超级管理员)
- Admin / 123456 (系统管理员)
- User / 123456 (普通用户)

## 文档
- [快速开始](docs/快速开始.md)
- [用户中心功能说明](docs/用户中心功能说明.md)
- [部署指南](docs/DEPLOYMENT.md)
- [贡献指南](CONTRIBUTING.md)

## 下载
- [Source code (zip)](https://github.com/lujing-jlu/art-design-pro-rs/archive/refs/tags/v$VERSION.zip)
- [Source code (tar.gz)](https://github.com/lujing-jlu/art-design-pro-rs/archive/refs/tags/v$VERSION.tar.gz)
EOF

echo -e "${GREEN}✓ 已生成 RELEASE_NOTES.md${NC}"

echo ""
echo "✅ 发布准备完成！"
echo "=================="
echo ""
echo "下一步操作:"
echo "1. 检查 RELEASE_NOTES.md"
echo "2. 提交所有更改: git add . && git commit -m 'chore: prepare for release v$VERSION'"
echo "3. 创建标签: git tag -a v$VERSION -m 'Release v$VERSION'"
echo "4. 推送到 GitHub: git push origin main --tags"
echo "5. 在 GitHub 上创建 Release"
echo ""
echo "📚 更多信息请查看 RELEASE_CHECKLIST.md"
echo ""
