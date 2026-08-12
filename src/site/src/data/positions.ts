export type Category = '技术类' | '产品类' | '市场类' | '职能类'

export interface Position {
  name: string
  category: Category
  duty: string
}

export const positions: Position[] = [
  {
    name: '数据工程师',
    category: '技术类',
    duty: '负责系统架构设计、软件编程、算法开发、技术创新及相关技术支持工作。',
  },
  {
    name: '产品经理',
    category: '产品类',
    duty: '负责产品规划、需求分析、产品设计、产品运营、项目管理及产品市场推广策略制定等工作。',
  },
  {
    name: '商务经理',
    category: '市场类',
    duty: '负责市场调研、品牌推广、客户关系管理、市场营销策划及销售渠道建设等工作。',
  },
  {
    name: '人事经理',
    category: '职能类',
    duty: '负责财务管理、法务管理、人力资源及其他职能支持工作。',
  },
]
