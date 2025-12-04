# 贡献指南

感谢你考虑为 Art Design Pro RS 做出贡献！

## 如何贡献

### 报告 Bug

如果你发现了 bug，请创建一个 issue 并包含以下信息：

1. **Bug 描述**：清晰简洁地描述问题
2. **复现步骤**：详细的复现步骤
3. **期望行为**：你期望发生什么
4. **实际行为**：实际发生了什么
5. **环境信息**：
   - 操作系统
   - Rust 版本
   - Node.js 版本
   - 浏览器版本（如果是前端问题）

### 提出新功能

如果你有新功能的想法：

1. 先创建一个 issue 讨论这个功能
2. 说明为什么需要这个功能
3. 描述你期望的实现方式
4. 等待维护者的反馈

### 提交代码

1. **Fork 项目**
   ```bash
   # 在 GitHub 上 fork 项目
   git clone https://github.com/your-username/art-design-pro-rs.git
   cd art-design-pro-rs
   ```

2. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/your-bug-fix
   ```

3. **编写代码**
   - 遵循项目的代码风格
   - 添加必要的注释
   - 确保代码通过所有测试

4. **提交代码**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   # 或
   git commit -m "fix: fix your bug description"
   ```

   提交信息格式：
   - `feat:` 新功能
   - `fix:` Bug 修复
   - `docs:` 文档更新
   - `style:` 代码格式调整
   - `refactor:` 代码重构
   - `test:` 测试相关
   - `chore:` 构建/工具相关

5. **推送到 GitHub**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **创建 Pull Request**
   - 在 GitHub 上创建 PR
   - 填写 PR 模板
   - 等待代码审查

## 开发规范

### 后端（Rust）

1. **代码风格**
   ```bash
   cargo fmt
   cargo clippy
   ```

2. **测试**
   ```bash
   cargo test
   cargo check
   ```

3. **命名规范**
   - 使用 snake_case 命名函数和变量
   - 使用 PascalCase 命名类型和 trait
   - 使用有意义的名称

### 前端（Vue 3 + TypeScript）

1. **代码风格**
   ```bash
   pnpm lint
   pnpm lint:prettier
   ```

2. **类型检查**
   ```bash
   pnpm build  # 会运行 vue-tsc
   ```

3. **命名规范**
   - 组件使用 PascalCase
   - 函数和变量使用 camelCase
   - 常量使用 UPPER_SNAKE_CASE

## 代码审查标准

你的 PR 需要满足以下条件才能被合并：

1. ✅ 代码风格符合规范
2. ✅ 所有测试通过
3. ✅ 没有编译警告
4. ✅ 添加了必要的文档
5. ✅ 没有破坏现有功能
6. ✅ 至少一位维护者审查通过

## 开发环境设置

### 后端

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装开发工具
cargo install cargo-watch

# 启动开发服务器（热重载）
cd backend
cargo watch -x 'run -p api'
```

### 前端

```bash
# 安装 pnpm
npm install -g pnpm

# 安装依赖
cd frontend
pnpm install

# 启动开发服务器
pnpm dev
```

## 测试

### 后端测试

```bash
cd backend

# 运行所有测试
cargo test

# 运行 API 测试
./test_api.sh

# 运行用户中心测试
./test_user_center.sh
```

### 前端测试

```bash
cd frontend

# 类型检查
pnpm build

# 代码检查
pnpm lint
```

## 文档

如果你的更改涉及到：

- 新功能：更新 README.md 和相关文档
- API 变更：更新 API 文档
- 配置变更：更新配置说明

## 问题和讨论

- 使用 GitHub Issues 报告问题
- 使用 GitHub Discussions 进行讨论
- 加入我们的社区（如果有）

## 行为准则

- 尊重所有贡献者
- 保持友好和专业
- 接受建设性的批评
- 关注对项目最有利的事情

## 许可证

通过贡献代码，你同意你的贡献将在 MIT 许可证下发布。

## 感谢

感谢所有为这个项目做出贡献的人！

---

如有任何问题，请随时创建 issue 或联系维护者。
