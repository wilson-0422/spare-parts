# 备品备件全生命周期管理系统

## 项目简介

备品备件全生命周期管理系统是一个基于 Rust/Axum/SQLite/Tera 构建的 Web 应用，用于管理工业备品备件从入厂到报废的全生命周期。系统涵盖原料入厂登记、生产领用管理、边角料处置追踪、库存预警监控和供应商对账等核心业务场景，帮助企业实现备品备件的精细化管理。

## 适用场景

- 制造业工厂备品备件仓库管理
- 设备维修部门零配件领用追踪
- 供应链管理中的供应商对账
- 仓储管理中的库存预警与补货
- 生产车间边角料处置与回收管理

## 核心功能

### 1. 原料入厂
- 备品备件信息录入（名称、分类、规格、单位）
- 库存数量与安全库存设置
- 单价管理与供应商关联
- 原料信息编辑与详情查看

### 2. 生产领用
- 领用申请提交（选择原料、填写数量与用途）
- 自动扣减库存数量
- 领用部门与申请人记录
- 领用状态管理（待审批/已批准/已拒绝）

### 3. 边角料处置
- 边角料处置记录（处置原因、处理人）
- 自动扣减库存数量
- 处置状态追踪
- 处置详情查看

### 4. 库存预警
- 实时库存列表展示（含库存金额计算）
- 安全库存阈值设置
- 低库存自动预警（库存 ≤ 安全库存）
- 预警列表按缺口数量排序

### 5. 供应商对账
- 供应商信息管理（联系人、电话、地址）
- 供货种类与金额统计
- 供货明细对账报表
- 对账金额自动汇总

### 6. 系统仪表盘
- 物料总数、库存预警数、库存总金额、供应商数量统计
- 最近领用记录
- 最近处置记录
- 库存预警提醒

## 技术栈

| 技术 | 说明 |
|------|------|
| Rust | 系统编程语言，高性能安全 |
| Axum 0.7 | Web 框架，异步路由 |
| SQLite (rusqlite) | 嵌入式数据库，零配置 |
| Tera | 模板引擎，服务端渲染 |
| tower-sessions | 会话管理，Cookie 认证 |
| bcrypt | 密码加密 |
| tower-http | 静态文件服务 |

## 目录结构

```
repo/
├── Cargo.toml
├── .gitignore
├── README.md
├── src/
│   ├── main.rs                    # 程序入口，路由配置
│   ├── config/
│   │   ├── mod.rs
│   │   ├── app.rs                 # 应用状态定义
│   │   ├── database.rs            # 数据库初始化
│   │   └── seed.rs                # 种子数据
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs                # 认证处理器
│   │   ├── home.rs                # 首页与仪表盘
│   │   ├── material.rs            # 原料管理处理器
│   │   ├── requisition.rs         # 生产领用处理器
│   │   ├── scrap.rs               # 边角料处置处理器
│   │   ├── inventory.rs           # 库存管理处理器
│   │   └── supplier.rs            # 供应商管理处理器
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs                # 认证中间件
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs                # 用户模型
│   │   ├── material.rs            # 原料模型
│   │   ├── requisition.rs         # 领用模型
│   │   ├── scrap.rs               # 处置模型
│   │   └── supplier.rs            # 供应商模型
│   └── services/
│       ├── mod.rs
│       ├── user_service.rs        # 用户服务
│       ├── material_service.rs    # 原料服务
│       ├── requisition_service.rs # 领用服务
│       ├── scrap_service.rs       # 处置服务
│       ├── inventory_service.rs   # 库存服务
│       └── supplier_service.rs    # 供应商服务
├── templates/
│   ├── base.html                  # 基础模板
│   ├── index.html                 # 首页
│   ├── partials/
│   │   ├── header.html            # 页头导航
│   │   └── footer.html            # 页脚
│   ├── auth/
│   │   ├── login.html             # 登录页
│   │   └── register.html          # 注册页
│   ├── materials/
│   │   ├── list.html              # 原料列表
│   │   ├── detail.html            # 原料详情
│   │   ├── create.html            # 新增原料
│   │   └── edit.html              # 编辑原料
│   ├── requisitions/
│   │   ├── list.html              # 领用列表
│   │   ├── create.html            # 新建领用
│   │   └── detail.html            # 领用详情
│   ├── scraps/
│   │   ├── list.html              # 处置列表
│   │   ├── create.html            # 新建处置
│   │   └── detail.html            # 处置详情
│   ├── inventory/
│   │   ├── list.html              # 库存列表
│   │   └── warning.html           # 库存预警
│   ├── suppliers/
│   │   ├── list.html              # 供应商列表
│   │   ├── detail.html            # 供应商详情
│   │   └── reconcile.html         # 供应商对账
│   └── dashboard/
│       └── overview.html          # 仪表盘
└── static/
    ├── css/
    │   └── style.css              # 样式表
    └── js/
        └── main.js                # 前端脚本
```

## Docker 启动方式

### 构建镜像

```bash
cd spare-parts
docker build -t spare-parts .
```

### 运行容器

```bash
docker run -d \
  --name spare-parts \
  -p 3000:3000 \
  -e SSH_ENABLE=false \
  spare-parts
```

### 启用 SSH 插件

```bash
docker run -d \
  --name spare-parts \
  -p 3000:3000 \
  -p 2222:22 \
  -e SSH_ENABLE=true \
  -e SSH_PUBLIC_KEY="ssh-rsa AAAAB3...your-key" \
  spare-parts
```

### 访问应用

浏览器打开 `http://localhost:3000`

## 本地启动方式

### 环境要求

- Rust 1.77+
- GCC（用于编译 SQLite）

### 启动步骤

```bash
cd spare-parts/repo
cargo run --release
```

应用将在 `http://0.0.0.0:3000` 启动。

### 开发模式

```bash
cd spare-parts/repo
cargo run
```

## 默认账号

| 用户名 | 密码 | 角色 |
|--------|------|------|
| admin | admin123 | 管理员 |
| zhangsan | 123456 | 普通用户 |
| lisi | 123456 | 普通用户 |

## 可扩展方向

1. **审批流程增强** - 实现多级审批、审批意见记录、审批历史追溯
2. **数据导出** - 支持 Excel/PDF 格式的库存报表、对账单导出
3. **采购管理** - 新增采购申请、采购订单、到货验收流程
4. **条码/二维码** - 物料条码管理，扫码入库出库
5. **API 接口** - 提供 RESTful API，支持移动端和第三方系统集成
6. **数据可视化** - 库存趋势图、领用统计图、供应商供货分析
7. **消息通知** - 库存预警邮件/短信通知、审批提醒
8. **多仓库管理** - 支持多仓库、库位管理
9. **权限细化** - 基于角色的访问控制（RBAC），菜单级权限
10. **审计日志** - 操作审计追踪，数据变更记录
