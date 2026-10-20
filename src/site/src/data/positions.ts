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

export const positions: Position[] = [
  // 实习
  {
    name: '数据工程师',
    employment: '实习',
    category: '技术类',
    duty: '负责系统架构设计、软件编程、算法开发、技术创新及相关技术支持工作。',
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
  {
    name: '执行助理',
    employment: '实习',
    category: '管理类',
    duty: '负责高管日程与会议组织、任务跟踪与落实反馈、重要事项跨部门协调等工作。',
    employerId: 'qttech',
  },
  // 兼职
  {
    name: '销售经理',
    employment: '兼职',
    category: '市场类',
    duty: '负责销售量潮全部定制服务，拓展客户、获取线索或促成成交，按客户实际付款比例分成。',
    employerId: 'qttech',
  },
]
