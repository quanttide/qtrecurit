export type Employment = '全职' | '实习' | '兼职'

export type Category = '技术类' | '产品类' | '市场类' | '职能类' | '管理类' | '科研类'

export interface Position {
  name: string
  employment: Employment
  category: Category
  duty: string
  // 合作雇主岗位挂 employerId；量潮自营岗位不填
  employerId?: string
  salary?: string
}

// 2026-11 政策：量潮科技只放高级岗位（职业身份），实训基地全量设置并细分（手艺技能）
export const positions: Position[] = [
  // 量潮科技——正式实习
  {
    name: '执行助理',
    employment: '实习',
    category: '管理类',
    duty: '负责高管日程与会议组织、任务跟踪与落实反馈、重要事项跨部门协调等工作。',
    employerId: 'qttech',
  },
  {
    name: '架构师',
    employment: '实习',
    category: '技术类',
    duty: '负责系统整体架构设计、技术选型与演进路线、关键技术难题攻关及技术规范制定等工作。',
    employerId: 'qttech',
  },
  {
    name: '产品经理',
    employment: '实习',
    category: '产品类',
    duty: '负责产品规划、需求分析、产品设计、产品运营、项目管理及产品市场推广策略制定等工作。',
    employerId: 'qttech',
  },
  {
    name: '商务经理',
    employment: '实习',
    category: '市场类',
    duty: '负责市场调研、品牌推广、客户关系管理、市场营销策划及销售渠道建设等工作。',
    employerId: 'qttech',
  },
  {
    name: '人事经理',
    employment: '实习',
    category: '职能类',
    duty: '负责财务管理、法务管理、人力资源及其他职能支持工作。',
    employerId: 'qttech',
  },

  // 量潮实训基地——教学实习：公司岗位全量设置，另按手艺细分
  {
    name: '执行助理',
    employment: '实习',
    category: '管理类',
    duty: '负责高管日程与会议组织、任务跟踪与落实反馈、重要事项跨部门协调等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '架构师',
    employment: '实习',
    category: '技术类',
    duty: '负责系统整体架构设计、技术选型与演进路线、关键技术难题攻关及技术规范制定等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '产品经理',
    employment: '实习',
    category: '产品类',
    duty: '负责产品规划、需求分析、产品设计、产品运营、项目管理及产品市场推广策略制定等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '商务经理',
    employment: '实习',
    category: '市场类',
    duty: '负责市场调研、品牌推广、客户关系管理、市场营销策划及销售渠道建设等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '人事经理',
    employment: '实习',
    category: '职能类',
    duty: '负责财务管理、法务管理、人力资源及其他职能支持工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '数据工程师',
    employment: '实习',
    category: '技术类',
    duty: '负责系统架构设计、软件编程、算法开发、技术创新及相关技术支持工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '全栈工程师',
    employment: '实习',
    category: '技术类',
    duty: '负责前后端功能的设计与开发、联调与交付、线上问题排查及全链路技术支持等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '客户经理',
    employment: '实习',
    category: '市场类',
    duty: '负责客户关系维护与需求挖掘、合作方案沟通、项目跟进与续约促成等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '品牌经理',
    employment: '实习',
    category: '市场类',
    duty: '负责品牌定位与形象管理、内容与传播策划、品牌资产维护及跨渠道一致性等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '营销经理',
    employment: '实习',
    category: '市场类',
    duty: '负责市场分析与营销策略制定、活动与增长策划、渠道投放与效果评估等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '公关经理',
    employment: '实习',
    category: '市场类',
    duty: '负责对外关系与媒体沟通、舆情监测与危机应对、公司对外发声与公共形象维护等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '财务经理',
    employment: '实习',
    category: '职能类',
    duty: '负责账务与核算管理、预算编制与成本控制、税务申报及财务报表等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '法务经理',
    employment: '实习',
    category: '职能类',
    duty: '负责合同审核与法律风险防控、合规事务对接、纠纷处理及法律事务管理等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '行政经理',
    employment: '实习',
    category: '职能类',
    duty: '负责办公环境与行政事务管理、制度与流程执行、后勤保障及跨部门行政协调等工作。',
    employerId: 'quanttide-academy',
  },
]
