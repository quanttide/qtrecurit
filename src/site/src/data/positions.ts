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

// 2026-11：量潮科技岗位以《量潮科技 2026 年 11 月线上实习招聘》公告为准（均为线上实习，起步 3 个月）；
// 实训基地＝教学实习初级岗（助理与工程师），两主体原则上零同名岗
 export const positions: Position[] = [
  // 量潮科技——正式实习（公告 8 岗）
  {
    name: '数据工程师',
    employment: '实习',
    category: '技术类',
    duty: '参与公司内外部项目开发，独立承担模块开发；以 AI 编程工具为主力优化研发工作流；参与基于 LLM 的业务场景探索与功能研发等工作。',
    employerId: 'qttech',
  },
  {
    name: '全栈工程师',
    employment: '实习',
    category: '技术类',
    duty: '围绕业务需求完成系统功能的设计、开发与迭代，覆盖前端页面、后端服务与 API 的开发联调及系统集成等工作。',
    employerId: 'qttech',
  },
  {
    name: '商务经理',
    employment: '实习',
    category: '市场类',
    duty: '主动挖掘企业客户的潜在需求与商机线索，参与商务全流程（背景调研、需求对接、方案准备、报价谈判及签约），维护客户关系与二次合作机会等工作。',
    employerId: 'qttech',
  },
  {
    name: '人事经理',
    employment: '实习',
    category: '职能类',
    duty: '负责公司全周期招聘工作（需求沟通、渠道运营、简历筛选、面试安排及薪酬谈判），协助企业文化活动与雇主品牌产出等工作。',
    employerId: 'qttech',
  },
  {
    name: '产品策划',
    employment: '实习',
    category: '产品类',
    duty: '总结现实问题并显性化为产品方案，负责产品从 0 到 1 的规划与落地，协调技术、市场、职能团队推动按计划交付等工作。',
    employerId: 'qttech',
  },
  {
    name: '产品运营',
    employment: '实习',
    category: '产品类',
    duty: '以用户视角识别体验问题并推动产研改进，负责社群与新媒体等用户触达渠道运营，输出运营复盘反哺产品迭代等工作。',
    employerId: 'qttech',
  },
  {
    name: '行政助理',
    employment: '实习',
    category: '职能类',
    duty: '协助日常行政事务（会议安排、日程协调、通知传达、资料整理），维护知识资产归档，跟进跨团队协作事项等工作。',
    employerId: 'qttech',
  },
  {
    name: '客户经理',
    employment: '实习',
    category: '市场类',
    duty: '深入了解客户业务与实际需求，维护客户关系，跟进沟通与合作进度并协调内部资源推动落地，识别续约与拓展机会等工作。',
    employerId: 'qttech',
  },

  // 量潮实训基地——教学实习：全部初级岗；与公司按手艺分层配对（助理 vs 经理），技术手艺由工程师岗承接
  {
    name: '产品助理',
    employment: '实习',
    category: '产品类',
    duty: '负责产品规划、需求分析、产品设计、产品运营、项目管理及产品市场推广策略制定等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '商务助理',
    employment: '实习',
    category: '市场类',
    duty: '负责市场调研、品牌推广、客户关系管理、市场营销策划及销售渠道建设等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '人事助理',
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
    name: '客户助理',
    employment: '实习',
    category: '市场类',
    duty: '负责客户关系维护与需求挖掘、合作方案沟通、项目跟进与续约促成等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '品牌助理',
    employment: '实习',
    category: '市场类',
    duty: '负责品牌定位与形象管理、内容与传播策划、品牌资产维护及跨渠道一致性等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '营销助理',
    employment: '实习',
    category: '市场类',
    duty: '负责市场分析与营销策略制定、活动与增长策划、渠道投放与效果评估等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '公关助理',
    employment: '实习',
    category: '市场类',
    duty: '负责对外关系与媒体沟通、舆情监测与危机应对、公司对外发声与公共形象维护等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '财务助理',
    employment: '实习',
    category: '职能类',
    duty: '负责账务与核算管理、预算编制与成本控制、税务申报及财务报表等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '法务助理',
    employment: '实习',
    category: '职能类',
    duty: '负责合同审核与法律风险防控、合规事务对接、纠纷处理及法律事务管理等工作。',
    employerId: 'quanttide-academy',
  },
  {
    name: '行政助理',
    employment: '实习',
    category: '职能类',
    duty: '负责办公环境与行政事务管理、制度与流程执行、后勤保障及跨部门行政协调等工作。',
    employerId: 'quanttide-academy',
  },
]