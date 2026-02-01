# rust-ecommerce 需求文档

## Project Overview

**项目名称**: rust-ecommerce
**项目类型**: 全栈 Web 电商平台
**技术栈**: Rust (Axum + Yew + WASM + SQLite)
**目标**: 构建一个功能完整的电商平台 MVP

---

## Actors

| Actor | Description | Permissions |
|-------|-------------|-------------|
| Guest | 未登录访客 | 浏览商品、查看详情 |
| User | 注册用户 | 购物车、下单、查看订单 |
| Admin | 管理员 | 商品 CRUD、订单管理 |

---

## Functional Requirements

### FR-001: 商品展示
- **描述**: 商品浏览和搜索系统
- **Actor**: Guest, User, Admin
- **Priority**: P1 (Must)
- **子功能**:
  - 商品列表页面 (分页展示，每页 12 个)
  - 商品详情页面 (名称、描述、价格、库存、图片)
  - 商品分类筛选 (按类别过滤)
  - 商品搜索 (按名称模糊搜索)
  - 价格排序 (升序/降序)

### FR-002: 购物车
- **描述**: 用户购物车管理
- **Actor**: User
- **Priority**: P1 (Must)
- **子功能**:
  - 添加商品到购物车 (指定数量)
  - 从购物车移除商品
  - 修改购物车商品数量
  - 购物车总价计算 (实时更新)
  - 购物车持久化 (用户登录后保留)
  - 库存检查 (添加时验证库存)

### FR-003: 订单管理
- **描述**: 订单创建和管理
- **Actor**: User, Admin
- **Priority**: P1 (Must)
- **子功能**:
  - 从购物车创建订单
  - 订单列表页面 (用户查看自己的订单)
  - 订单详情页面 (商品、数量、总价、状态)
  - 订单状态流转 (Pending → Paid → Shipped → Delivered)
  - 取消订单 (仅 Pending 状态可取消)
  - 管理员查看所有订单

### FR-004: 用户认证
- **描述**: 用户注册和登录系统
- **Actor**: Guest, User
- **Priority**: P1 (Must)
- **子功能**:
  - 用户注册 (用户名、邮箱、密码)
  - 用户登录 (邮箱 + 密码)
  - JWT Token 认证
  - 用户登出 (Token 失效)
  - 密码加密存储 (bcrypt/argon2)
  - 登录状态持久化 (LocalStorage)

### FR-005: 商品管理 (Admin)
- **描述**: 管理员商品 CRUD
- **Actor**: Admin
- **Priority**: P2 (Should)
- **子功能**:
  - 添加新商品 (名称、描述、价格、库存、分类)
  - 编辑商品信息
  - 删除商品 (软删除)
  - 商品上架/下架
  - 库存管理

---

## Non-Functional Requirements

### NFR-001: 性能
- 页面加载时间 < 2 秒
- API 响应时间 < 500ms
- 支持 100 并发用户

### NFR-002: 安全
- 密码加密存储
- JWT Token 过期时间 24 小时
- SQL 注入防护 (参数化查询)
- XSS 防护

### NFR-003: 可用性
- 响应式设计 (支持移动端)
- 错误提示友好
- 加载状态指示

---

## Constraints

### 技术约束
- 使用 Rust 语言
- 后端框架: Axum
- 前端框架: Yew (WASM)
- 数据库: SQLite (开发简单，可后续迁移)

### 资源约束
- MVP 版本，核心功能优先
- 单人开发

---

## Assumptions

1. 用户有现代浏览器支持 WASM
2. 开发环境已安装 Rust 工具链
3. 不需要支付系统集成 (模拟支付)
4. 图片使用 URL 链接，不做上传

---

## Out of Scope

- 支付网关集成
- 邮件通知
- 多语言支持
- 商品评论系统
- 优惠券系统
- 物流追踪

---

*需求文档版本: 1.0.0*
*创建时间: 2026-02-01*
