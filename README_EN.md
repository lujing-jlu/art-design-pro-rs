# Art Design Pro RS

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![Vue](https://img.shields.io/badge/Vue-3.5+-brightgreen.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.6+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

A Modern Admin Dashboard System Built with Rust + Vue 3

[Live Demo](#) | [Quick Start](docs/快速开始.md) | [Documentation](docs/) | [Contributing](CONTRIBUTING.md)

[中文文档](README.md)

</div>

---

## 📖 Introduction

Art Design Pro RS is a full-stack admin dashboard system featuring a high-performance Rust backend and a modern Vue 3 + TypeScript frontend. It provides complete enterprise-level features including user management, role-based access control, and menu management.

**Features**:
- 🚀 **High Performance**: Rust backend for ultimate performance and memory safety
- 🎨 **Modern**: Vue 3 + TypeScript + Vite for excellent developer experience
- 🔐 **Secure**: JWT authentication, Argon2 password hashing, brute-force protection
- 📦 **Ready to Use**: Complete CRUD, permission control, dynamic routing
- 🛠️ **Easy to Extend**: Clear architecture, modular design

## ✨ Features

### Backend (Rust)
- **Framework**: Actix Web, SeaORM, SQLite, JWT, Argon2
- **Architecture**: Workspace structure with `api` (includes startup initialization in `api/src/db_init.rs`), `common`, `service`, `entity`
- **Features**:
  - Authentication: Login, Register, Password Reset
  - User Center: Profile Management, Password Change, Avatar Upload
  - System Management: User CRUD, Role CRUD, Menu CRUD
  - Unified Response: `{ code, msg, data }`, 401 triggers auto logout
- **Database**: SQLite by default (supports PostgreSQL/MySQL)
- **Test Scripts**: `test_api.sh`, `test_user_center.sh`

### Frontend (Vue 3)
- **Framework**: Vue 3 + TypeScript + Vite + Element Plus
- **Features**:
  - Backend mode with real API integration
  - Authentication pages
  - User center with profile editing
  - System management (Users, Roles, Menus)
  - Dynamic routing and menu loading
- **Development**: Hot reload, TypeScript support

## 🚀 Quick Start

### Prerequisites

**Backend**:
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- SQLite 3

**Frontend**:
- Node.js 20.19.0+
- pnpm 8.8.0+

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/lujing-jlu/art-design-pro-rs.git
cd art-design-pro-rs
```

2. **Start Backend**
```bash
cd backend
cargo run -p api
# Backend runs at http://127.0.0.1:9123
```

3. **Start Frontend** (in a new terminal)
```bash
cd frontend
pnpm install
pnpm dev
# Frontend runs at http://localhost:4000
```

### Test Accounts

All passwords are `123456`:

| Username | Role | Description |
|----------|------|-------------|
| Super | R_SUPER | Super Administrator |
| Admin | R_ADMIN | System Administrator |
| User | R_USER | Regular User |

## 📚 Documentation

- [Quick Start Guide](docs/快速开始.md)
- [User Center Features](docs/用户中心功能说明.md)
- [Frontend-Backend Integration](docs/前后端对接说明.md)
- [Implementation Summary](docs/实现总结.md)

## 🔌 Main APIs

### Authentication
- `POST /api/auth/login` - User login
- `POST /api/auth/register` - User registration
- `POST /api/auth/forget-password` - Password reset
- `GET /api/user/info` - Get user info

### User Center
- `PUT /api/user/profile` - Update profile
- `PUT /api/user/password` - Change password
- `POST /api/user/avatar` - Upload avatar

### User Management
- `GET /api/user/list` - Get user list
- `POST /api/user` - Create user
- `PUT /api/user` - Update user
- `DELETE /api/user/{id}` - Delete user

### Role Management
- `GET /api/role/list` - Get role list
- `POST /api/role` - Create role
- `PUT /api/role` - Update role
- `DELETE /api/role/{id}` - Delete role

### Menu Management
- `GET /api/v3/system/menus/simple` - Get menu tree
- `GET /api/menu/list` - Get all menus (admin)
- `POST /api/menu` - Create menu
- `PUT /api/menu` - Update menu
- `DELETE /api/menu/{id}` - Delete menu

## 🛠️ Development

### Backend Development

```bash
cd backend

# Hot reload
cargo install cargo-watch
cargo watch -x 'run -p api'

# Code check
cargo check
cargo clippy

# Format
cargo fmt

# Test
cargo test
./test_api.sh
./test_user_center.sh
```

### Frontend Development

```bash
cd frontend

# Lint
pnpm lint
pnpm lint:prettier

# Type check
pnpm build

# Format
pnpm lint:prettier
```

## 📦 Production Build

### Backend

```bash
cd backend
cargo build --release
./target/release/api
```

### Frontend

1. Update API URL in `.env.production`:
```
VITE_API_URL=https://your-api-domain.com
```

2. Build:
```bash
cd frontend
pnpm build
```

3. Deploy the `frontend/dist` directory to your static server.

## 🗺️ Roadmap

- [x] Basic authentication system
- [x] User management
- [x] Role management
- [x] Menu management
- [x] User center
- [ ] Department management
- [ ] Operation logs
- [ ] Data dictionary
- [ ] File management
- [ ] System monitoring
- [ ] Online users
- [ ] Scheduled tasks

## 🤝 Contributing

Contributions are welcome! Please check out the [Contributing Guide](CONTRIBUTING.md).

## 📄 License

This project is licensed under the [MIT License](LICENSE).

The frontend is based on [Art Design Pro](https://github.com/Daymychen/art-design-pro) and follows its open source license.

## 🙏 Acknowledgments

- [Art Design Pro](https://github.com/Daymychen/art-design-pro) - Frontend template
- [Actix Web](https://actix.rs/) - Rust web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) - Rust ORM
- [Vue 3](https://vuejs.org/) - Progressive JavaScript framework
- [Element Plus](https://element-plus.org/) - Vue 3 component library

## 📞 Contact

- Submit Issues: [GitHub Issues](https://github.com/lujing-jlu/art-design-pro-rs/issues)
- Discussions: [GitHub Discussions](https://github.com/lujing-jlu/art-design-pro-rs/discussions)

## ⭐ Star History

If this project helps you, please give it a Star ⭐

[![Star History Chart](https://api.star-history.com/svg?repos=lujing-jlu/art-design-pro-rs&type=Date)](https://star-history.com/#lujing-jlu/art-design-pro-rs&Date)
