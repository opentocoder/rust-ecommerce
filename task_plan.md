# rust-ecommerce 任务计划

## 项目概述

**项目名称**: rust-ecommerce
**技术栈**: Rust, Axum, Yew, WASM, SQLite
**目标**: 构建全栈 Rust 电商平台 MVP
**需求文档**: docs/requirements.md

---

## Phase 1: 项目基础结构

### 1.1 创建 Workspace 成员目录
- **描述**: 创建 backend、frontend、shared 三个 crate 的目录结构
- **依赖**: 无
- **FR 映射**: 基础设施
- **验收标准**: 目录结构存在

### 1.2 配置各 Crate 的 Cargo.toml
- **描述**: 配置依赖
  - backend: axum, tokio, sqlx, serde, jsonwebtoken, argon2
  - frontend: yew, wasm-bindgen, gloo
  - shared: serde, thiserror
- **依赖**: 1.1
- **FR 映射**: 基础设施
- **验收标准**: `cargo check` 通过

### 1.3 创建基础入口文件
- **描述**: 创建 main.rs/lib.rs 入口
- **依赖**: 1.2
- **FR 映射**: 基础设施
- **验收标准**: `cargo build` 成功

---

## Phase 2: 共享数据模型 (FR-001, FR-002, FR-003, FR-004)

### 2.1 定义核心数据模型
- **描述**: 在 shared/src/models/ 定义:
  - Product (id, name, description, price, stock, category, image_url)
  - User (id, username, email, password_hash, role, created_at)
  - CartItem (user_id, product_id, quantity)
  - Order (id, user_id, status, total, created_at)
  - OrderItem (order_id, product_id, quantity, price)
- **依赖**: 1.3
- **FR 映射**: FR-001, FR-002, FR-003, FR-004
- **验收标准**: 模型定义完整，derive Serialize/Deserialize

### 2.2 定义 API 类型
- **描述**: 在 shared/src/api/ 定义:
  - 请求类型: LoginRequest, RegisterRequest, CreateOrderRequest
  - 响应类型: ProductResponse, CartResponse, OrderResponse
  - 错误类型: ApiError
- **依赖**: 2.1
- **FR 映射**: FR-001, FR-002, FR-003, FR-004
- **验收标准**: API 类型完整

---

## Phase 3: Backend 数据库层 (FR-001, FR-002, FR-003, FR-004)

### 3.1 数据库初始化
- **描述**: SQLite 连接池配置，迁移脚本
  - migrations/001_init.sql: users, products, cart_items, orders, order_items 表
- **依赖**: 2.2
- **FR 映射**: FR-001, FR-002, FR-003, FR-004
- **验收标准**: 数据库初始化成功，表结构正确

### 3.2 Repository 层
- **描述**: backend/src/db/ CRUD 操作
  - ProductRepository: list, get, search, filter_by_category
  - UserRepository: create, find_by_email
  - CartRepository: get_cart, add_item, remove_item, update_quantity
  - OrderRepository: create, list_by_user, get, update_status
- **依赖**: 3.1
- **FR 映射**: FR-001, FR-002, FR-003, FR-004
- **验收标准**: Repository 方法可用

---

## Phase 4: Backend API 路由 (FR-001, FR-002, FR-003, FR-004)

### 4.1 认证模块 (FR-004)
- **描述**: backend/src/auth/
  - POST /api/auth/register - 用户注册
  - POST /api/auth/login - 用户登录，返回 JWT
  - JWT 中间件验证
  - 密码加密 (argon2)
- **依赖**: 3.2
- **FR 映射**: FR-004
- **验收标准**: 注册/登录 API 工作正常

### 4.2 商品 API (FR-001)
- **描述**: backend/src/routes/products.rs
  - GET /api/products - 商品列表 (分页、排序)
  - GET /api/products/:id - 商品详情
  - GET /api/products/search?q= - 搜索
  - GET /api/products/category/:cat - 分类筛选
- **依赖**: 3.2
- **FR 映射**: FR-001
- **验收标准**: 商品 API 返回正确数据

### 4.3 购物车 API (FR-002)
- **描述**: backend/src/routes/cart.rs
  - GET /api/cart - 获取购物车
  - POST /api/cart - 添加商品
  - PUT /api/cart/:product_id - 更新数量
  - DELETE /api/cart/:product_id - 移除商品
- **依赖**: 4.1
- **FR 映射**: FR-002
- **验收标准**: 购物车 CRUD 工作正常

### 4.4 订单 API (FR-003)
- **描述**: backend/src/routes/orders.rs
  - POST /api/orders - 创建订单
  - GET /api/orders - 订单列表
  - GET /api/orders/:id - 订单详情
  - PUT /api/orders/:id/cancel - 取消订单
- **依赖**: 4.3
- **FR 映射**: FR-003
- **验收标准**: 订单 API 工作正常

### 4.5 服务器启动
- **描述**: 配置 Axum 服务器
  - CORS 配置
  - 路由注册
  - 错误处理
- **依赖**: 4.4
- **FR 映射**: 基础设施
- **验收标准**: `cargo run -p backend` 启动成功

---

## Phase 5: Frontend 基础 (FR-001, FR-002, FR-003, FR-004)

### 5.1 Yew 应用框架
- **描述**: 配置 Yew + Trunk
  - frontend/index.html
  - frontend/Trunk.toml
  - App 组件和路由
- **依赖**: 2.2
- **FR 映射**: 基础设施
- **验收标准**: `trunk serve` 成功

### 5.2 API 客户端
- **描述**: frontend/src/api/
  - HTTP 请求封装 (gloo-net)
  - Token 管理
  - 错误处理
- **依赖**: 5.1
- **FR 映射**: 基础设施
- **验收标准**: API 调用成功

### 5.3 状态管理
- **描述**: frontend/src/state/
  - AuthState: 用户登录状态
  - CartState: 购物车状态
  - 使用 yewdux 或 Context
- **依赖**: 5.2
- **FR 映射**: FR-002, FR-004
- **验收标准**: 状态管理工作正常

---

## Phase 6: Frontend 页面 (FR-001, FR-002, FR-003, FR-004)

### 6.1 认证页面 (FR-004)
- **描述**: frontend/src/pages/
  - LoginPage: 登录表单
  - RegisterPage: 注册表单
- **依赖**: 5.3
- **FR 映射**: FR-004
- **验收标准**: 登录/注册流程完整

### 6.2 商品页面 (FR-001)
- **描述**:
  - ProductListPage: 商品列表 (分页、筛选、排序)
  - ProductDetailPage: 商品详情
- **依赖**: 5.3
- **FR 映射**: FR-001
- **验收标准**: 商品展示正确

### 6.3 购物车页面 (FR-002)
- **描述**:
  - CartPage: 购物车列表、数量修改、删除、结算
- **依赖**: 6.2
- **FR 映射**: FR-002
- **验收标准**: 购物车功能完整

### 6.4 订单页面 (FR-003)
- **描述**:
  - OrderListPage: 订单列表
  - OrderDetailPage: 订单详情
- **依赖**: 6.3
- **FR 映射**: FR-003
- **验收标准**: 订单功能完整

---

## Phase 7: 测试与完善

### 7.1 Backend 单元测试
- **描述**: Repository 和 Service 测试
- **依赖**: 4.5
- **FR 映射**: 质量保证
- **验收标准**: 测试通过

### 7.2 API 集成测试
- **描述**: API 端点测试
- **依赖**: 7.1
- **FR 映射**: 质量保证
- **验收标准**: 测试通过

### 7.3 种子数据
- **描述**: 初始化测试数据
- **依赖**: 7.2
- **FR 映射**: 质量保证
- **验收标准**: 数据正确加载

---

## 任务统计

| Phase | 任务数 | 状态 |
|-------|--------|------|
| Phase 1 | 3 | pending |
| Phase 2 | 2 | pending |
| Phase 3 | 2 | pending |
| Phase 4 | 5 | pending |
| Phase 5 | 3 | pending |
| Phase 6 | 4 | pending |
| Phase 7 | 3 | pending |
| **总计** | **22** | - |

---

*任务计划版本: 1.0.0*
*创建时间: 2026-02-01*
