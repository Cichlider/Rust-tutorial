# BusinessRadar - MSME商机雷达平台

**中英文版本 | English & Chinese Version**

## 📋 项目概述 | Project Overview

### 🇨🇳 中文版本
BusinessRadar是一个专为中小微企业(MSMEs)设计的商机发现平台，特别针对餐饮服务和个体经营者。该平台通过智能分析区域内即将举办的活动和节假日，为企业提供潜在的商业机会信息。

### 🇬🇧 English Version
BusinessRadar is a business opportunity discovery platform designed specifically for Micro, Small, and Medium Enterprises (MSMEs), particularly food services and individual businesses. The platform intelligently analyzes upcoming events and holidays within regions to provide potential business opportunity information for enterprises.

---

## 🎯 核心功能 | Core Features

### 📊 功能演示 | Functionality Demonstration
**PowerPoint演示文稿:** [查看演示](https://docs.google.com/presentation/d/1TkGB4P0EvWyFs2KsBtTQNaFSEUQ3saorDcyjx8ihf9U/edit?usp=sharing)

### 🎯 主要目标 | Main Objectives

#### 🇨🇳 帮助中小微企业：
1. **库存管理** - 合理安排库存，确保有足够商品应对潜在需求增长
2. **人员调度** - 调整员工排班，确保活动日有充足人手及时服务客户，避免因等待时间过长而流失客户  
3. **营销推广** - 向大量潜在客户推广和宣传业务

#### 🇬🇧 Help MSMEs to：
1. **Inventory Management** - Arrange stocks to ensure adequate supply for potential demand increases
2. **Staff Scheduling** - Adjust staff schedules to ensure sufficient personnel on event days for prompt customer service and prevent customer loss due to excessive wait times
3. **Marketing & Promotion** - Promote and advertise business to large numbers of potential customers

---

## 💼 使用场景 | Use Cases

### 🍽️ 餐厅案例 | Restaurant Example
**🇨🇳 中文：**
一家餐厅希望了解未来一周内1公里范围内的活动信息。系统每日更新本周剩余时间和下周的即将举办的活动，提醒餐厅检查库存、调整员工排班，并在活动当天加强促销。

**🇬🇧 English：**
A restaurant wants to stay updated about events in the upcoming week within 1 kilometer. Events are updated daily about upcoming events for the rest of the week and the following week, prompting checks on stocks, staff schedule adjustments, and increased promotions on event days.

### 🎨 艺术家案例 | Artist Example
**🇨🇳 中文：**
一位销售艺术作品的艺术家希望了解活动信息，特别是艺术相关活动和市集日。她选择接收10公里范围内提前1个月的相关活动信息，以便联系组织者并准备作品。

**🇬🇧 English：**
An artist selling art pieces wants updates about events, especially art-related events and market days. She chooses to receive updates about related events within a 10km radius and 1 month in advance, allowing her to reach out to organizers and prepare pieces.

---

## 🖥️ 界面设计 | Interface Design

### 📱 左侧面板 | Left Panel
- **🏢 Logo + [Go Pro按钮]**
- **📍 搜索栏：** "Your Location | 您的位置"
- **📏 距离设置：**
  - 拖动条控制距离范围
  - 手动输入距离数值
- **🏷️ 标签/商机分类**
- **📋 活动列表：** 显示根据相关性排序的前10个活动
  - 活动名称
  - 举办日期  
  - 距用户距离
  - 具体地址
  - 标签/商机类型
  - 相关链接
- **⭐ Pro版本提示：** "获得Pro版本即可获取更多活动信息 | Get Pro version for more event information"

### 🗺️ 右侧地图 | Right Side Map
- **📍 用户位置显示**
- **📌 活动地点标记**
- **🔍 点击显示具体地址**

### 👤 右上角功能 | Top Right Corner
- **🔐 登录按钮 | Login Button**
- **🌐 中英文切换按钮 | Language Toggle**

---

## 🛠️ 技术架构 | Technical Architecture

### 前端技术栈 | Frontend Tech Stack
- **Framework:** React.js / Vue.js
- **UI Library:** Tailwind CSS / Ant Design
- **Map Integration:** Google Maps API / Mapbox
- **State Management:** Redux / Vuex
- **Language:** TypeScript

### 后端技术栈 | Backend Tech Stack
- **Runtime:** Node.js
- **Framework:** Express.js / Nest.js
- **Database:** MongoDB / PostgreSQL
- **Authentication:** JWT
- **API:** RESTful API / GraphQL

### 第三方集成 | Third-party Integrations
- **地图服务 | Map Services:** Google Maps API, OpenStreetMap
- **活动数据源 | Event Data Sources:** Eventbrite API, Facebook Events, Local Government APIs
- **地理位置 | Geolocation:** HTML5 Geolocation API
- **支付系统 | Payment:** Stripe (Pro版本)

---

## 📊 数据模型 | Data Models

### 用户模型 | User Model
```json
{
  "userId": "string",
  "email": "string",
  "businessType": "string",
  "location": {
    "latitude": "number",
    "longitude": "number",
    "address": "string"
  },
  "preferences": {
    "radius": "number",
    "categories": ["string"],
    "advanceNotice": "number"
  },
  "isPro": "boolean"
}
```

### 活动模型 | Event Model
```json
{
  "eventId": "string",
  "title": "string",
  "description": "string",
  "date": "datetime",
  "location": {
    "latitude": "number",
    "longitude": "number",
    "address": "string"
  },
  "category": "string",
  "tags": ["string"],
  "expectedAttendance": "number",
  "businessOpportunity": "string"
}
```

---

## 🚀 安装和部署 | Installation & Deployment

### 环境要求 | Prerequisites
- Node.js 16+
- MongoDB 5.0+
- npm/yarn

### 本地开发 | Local Development
```bash
# 克隆项目
git clone https://github.com/your-org/business-radar.git

# 安装依赖
npm install

# 环境配置
cp .env.example .env

# 启动开发服务器
npm run dev
```

### 生产部署 | Production Deployment
```bash
# 构建项目
npm run build

# 启动生产服务器
npm run start
```

---

## 📱 功能特性 | Feature Highlights

### 🆓 免费版功能 | Free Version Features
- ✅ 基础活动搜索
- ✅ 10个活动结果显示
- ✅ 基本地图功能
- ✅ 距离筛选（最远5km）

### ⭐ Pro版功能 | Pro Version Features
- ✅ 无限活动结果
- ✅ 高级筛选选项
- ✅ 提前通知设置
- ✅ 数据导出功能
- ✅ 优先客户支持
- ✅ API访问权限

---

## 🌍 多语言支持 | Multi-language Support

目前支持的语言 | Currently Supported Languages:
- 🇨🇳 简体中文 (Simplified Chinese)
- 🇬🇧 English
- 🇪🇸 Español (计划中 | Planned)
- 🇫🇷 Français (计划中 | Planned)

---

## 📞 联系我们 | Contact Us

- **📧 Email:** support@businessradar.com
- **🌐 Website:** https://businessradar.com
- **💬 Support:** 24/7 在线客服 | 24/7 Online Support

---

## 📄 许可证 | License

MIT License - 详见 [LICENSE](LICENSE) 文件

---

## 🤝 贡献指南 | Contributing

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## 📈 路线图 | Roadmap

### 第一阶段 | Phase 1 (Q1 2025)
- [x] 基础平台开发
- [x] 地图集成
- [x] 用户认证系统

### 第二阶段 | Phase 2 (Q2 2025)
- [ ] Pro版本功能
- [ ] 移动应用开发
- [ ] API开放平台

### 第三阶段 | Phase 3 (Q3 2025)
- [ ] AI智能推荐
- [ ] 社区功能
- [ ] 国际化扩展

---

**⭐ 如果这个项目对你有帮助，请给我们一个Star！**
**⭐ If this project helps you, please give us a Star!**