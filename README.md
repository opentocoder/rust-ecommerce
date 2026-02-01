# RustShop - Full-Stack E-Commerce Platform

A complete e-commerce platform built entirely in Rust, featuring a high-performance Axum backend and a WebAssembly frontend powered by Yew.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust, Axum, SQLite, SQLx |
| Frontend | Rust, Yew, WASM, Trunk |
| Auth | JWT, Argon2 |
| Shared | Common types between backend & frontend |

## Features

- **User Authentication**: Register, login, logout with JWT tokens
- **Product Catalog**: Browse products with categories and search
- **Shopping Cart**: Add, update quantity, remove items
- **Order Management**: Create orders, view order history
- **Responsive UI**: Clean, modern interface

## Project Structure

```
rust-ecommerce/
├── backend/          # Axum REST API server
│   └── src/
│       ├── auth/     # JWT & password handling
│       ├── db/       # SQLite repositories
│       └── routes/   # API endpoints
├── frontend/         # Yew WASM application
│   └── src/
│       ├── api/      # HTTP client
│       ├── components/
│       ├── pages/
│       └── state/    # Auth & cart state
├── shared/           # Common types
│   └── src/
│       ├── api/      # Request/Response types
│       └── models/   # Data models
└── 验收/             # Acceptance test screenshots
```

## Quick Start

### Prerequisites

- Rust (latest stable)
- Trunk (`cargo install trunk`)
- wasm32 target (`rustup target add wasm32-unknown-unknown`)

### Run Backend

```bash
# Start the API server (port 3000)
cargo run -p backend

# Load seed data (12 sample products)
cargo run -p backend --bin seed
```

### Run Frontend

```bash
cd frontend
trunk serve  # Starts on port 8080
```

### Access

- **Frontend**: http://localhost:8080
- **Backend API**: http://localhost:3000
- **Health Check**: http://localhost:3000/health

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/auth/register` | POST | User registration |
| `/api/auth/login` | POST | User login |
| `/api/products` | GET | Product list |
| `/api/products/:id` | GET | Product detail |
| `/api/categories` | GET | Category list |
| `/api/cart` | GET/POST | Cart operations |
| `/api/cart/:id` | PUT/DELETE | Update/remove cart item |
| `/api/orders` | GET/POST | Order operations |
| `/api/orders/:id` | GET | Order detail |

## Screenshots

### Home Page
![Home Page](验收/01-首页.png)

### Product Catalog
![Product List](验收/02-商品列表.png)

### Product Detail
![Product Detail](验收/03-商品详情.png)

### User Authentication

| Login | Register |
|-------|----------|
| ![Login](验收/04-登录页面.png) | ![Register](验收/05-注册页面.png) |

### Registration Flow

| Fill Form | Error Handling | Success |
|-----------|----------------|---------|
| ![Fill](验收/06-注册表单填写.png) | ![Error](验收/07-注册错误提示.png) | ![Success](验收/08-注册成功已登录.png) |

### Shopping Flow

| Product (Logged In) | Add to Cart |
|---------------------|-------------|
| ![Product](验收/09-商品详情已登录.png) | ![Added](验收/10-添加购物车成功.png) |

### Cart Management

| Cart View | Update Quantity |
|-----------|-----------------|
| ![Cart](验收/11-购物车页面.png) | ![Update](验收/12-购物车数量更新.png) |

### Order Management

| Order Created | Order List | Logout |
|---------------|------------|--------|
| ![Created](验收/13-订单创建成功.png) | ![List](验收/14-订单列表.png) | ![Logout](验收/15-登出成功.png) |

## Test Results

| Category | Tests | Status |
|----------|-------|--------|
| Authentication | 5 | ✅ Pass |
| Products API | 4 | ✅ Pass |
| Cart API | 4 | ✅ Pass |
| Orders API | 3 | ✅ Pass |
| Server | 2 | ✅ Pass |
| Frontend Pages | 14 | ✅ Pass |
| **Total** | **32** | **✅ All Pass** |

## License

MIT

---

*Built with Rust, Axum & Yew*
