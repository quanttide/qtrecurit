export interface Employer {
  id: string
  name: string
  intro: string
  // 合作雇主的投递方式可能与量潮自营不同，按雇主配置
  applyEmail: string
  subjectFormat: string
}

export const employers: Employer[] = [
  {
    id: 'qttech',
    name: '量潮科技',
    intro:
      '作为一家制度创新实验室，量潮科技希望可以帮助人类更美好的协作，以促进解放全人类的创造力。主营业务包括大数据处理服务（量潮数据）、大数据技术课程（量潮课堂）与软件技术咨询（量潮咨询），是浙江理工大学计算机系大数据微专业机构授课方。',
    applyEmail: 'hr@quanttide.com',
    subjectFormat: '姓名-学校-应聘岗位',
  },
]
