import { positionCategories } from '../data/positions'

function Join() {
  return (
    <>
      <header>
        <h1>加入我们</h1>
        <p className="period">量潮期待您的加入</p>
      </header>

      <section className="join-section">
        <h2>公司简介</h2>
        <p>量潮科技是一家致力于为客户提供基于云原生标准的数据工程、数据分析和算法工程解决方案的数据科技公司。在 Serverless、云原生、DevOps 应用于软件工程、数据工程、算法工程、数据分析等领域拥有业界前沿的积累。主营业务是为科研团队和中小企业提供数据分析、数据处理和智能体定制软件服务；为个人提供高质量的 AIGC 在线课程，让 AI 技术助力各领域的生产。</p>
        <p>“引领定量分析时代潮流”是我们的品牌口号。我们专注于通过定量分析方法理解社会运行规律，创造打动人心的产品和服务，成为这个行业的探路者和领导者，推动整个行业的发展。</p>
      </section>

      <section className="join-section">
        <h2>招聘要求</h2>
        {positionCategories.map((c) => (
          <div key={c.name} className="position-category">
            <h3>{c.name}</h3>
            <p>{c.duty}</p>
            <p className="hourly-rate">{c.hourlyRate}</p>
          </div>
        ))}
      </section>

      <section className="join-section">
        <h2>报名方式</h2>
        <p>有意者请按要求完成求职信和简历，并将它们通过邮件发送到 hr@quanttide.com。</p>
        <p>邮件标题为 <strong>姓名-学校-应聘岗位</strong>，邮件正文写求职信，附件放简历。</p>
      </section>

      <section className="join-section">
        <h2>求职信</h2>
        <p>您的求职信中应注明意向的一个或多个职务类型（技术类、产品类、市场类和职能类）或具体岗位，并围绕以下方面阐述：</p>
        <ol>
          <li>您在目标应聘岗位的工作经验或参与项目经验；</li>
          <li>腾讯系协作软件使用经验（腾讯文档、企业微信、腾讯会议等）；</li>
          <li>从您的目标岗位角度谈谈 AI 工具辅助工作的经验和心得，对企业数据服务市场、科研数据服务市场、云服务市场的认识，对类似企业的看法。</li>
        </ol>
      </section>

      <section className="join-section">
        <h2>简历</h2>
        <p>您的简历应该尽可能简单标准地展示信息，准确真实地描述经历的具体过程和技能的具体水平，不使用花哨的模板。</p>
        <ol>
          <li>正式规范的给出您的基本信息：使用与本人对应的字母邮箱；使用正式照片；</li>
          <li>在教育经历中列出您的主要专业课或与意向岗位对口的专业课。</li>
          <li>亮点经历会帮助我们区分您和其他人：如果您有工作或实习经历，请详细阐述您在实习公司部门的具体职责和取得的成果，包括公司及部门的主要业务，个人在岗位上的具体工作和成果等；项目经历与实习经历类似，您需要介绍项目的目的和方法、您在项目中的工作内容和项目取得的成果；如果有担任学生组织或社团中的核心岗位，请简单描述下活动内容。其它亮点经历，请简要描述，介绍您的参与情况和工作量。</li>
          <li>我们着重在意的是您具体工作技能，请列举拥有的技能及水平，并给出相关的证书或证明。</li>
        </ol>
        <p>如果你需要了解如何制作简历，可以参考我们的公众号文章：<a href="https://mp.weixin.qq.com/s/40whc0s68XdSXZo8TqTfuA">大学生如何制作求职和升学简历？</a></p>
      </section>
    </>
  )
}

export default Join
