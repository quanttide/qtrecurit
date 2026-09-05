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
  {
    id: 'dmu',
    name: '大连医科大学课题组',
    intro:
      '课题组长期聚焦心血管疾病与代谢、神经免疫交互机制研究，综合运用免疫学、分子生物学、单细胞多组学、生物信息学等手段，围绕心肌纤维化、动脉粥样硬化、神经炎症等疾病中的免疫-间质/神经互作开展基础与转化研究。课题组依托大连医科大学附属第二医院，与海内外顶尖科研机构保持深度合作。',
    applyEmail: '1073926653@qq.com',
    subjectFormat: '应聘岗位（实验/生信）-姓名-毕业学校',
  },
]
