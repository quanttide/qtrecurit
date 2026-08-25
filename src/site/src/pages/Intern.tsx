import { positions } from '../data/positions'

const internPositions = positions.filter((p) => p.employment === '实习')

function Intern() {
  return (
    <>
      <section id="positions" className="join-section">
        <h2>实习岗位</h2>
        <div className="position-grid">
          {internPositions.map((p) => (
            <div key={p.name} className="position-card">
              <div className="position-head">
                <h3>{p.name}</h3>
                <span className="position-category">{p.category}</span>
              </div>
              <p>{p.duty}</p>
            </div>
          ))}
        </div>
      </section>

      <section id="apply" className="join-section">
        <h2>报名方式</h2>
        <p>有意者请按要求完成求职信和简历，并将它们通过邮件发送到 hr@quanttide.com。</p>
        <p>邮件标题为 <strong>姓名-学校-应聘岗位</strong>，邮件正文写求职信，附件放简历。</p>
        <p><a href="/intern/application">查看简历和求职信要求 →</a></p>
      </section>

      <section id="assessment" className="join-section">
        <h2>考核方式</h2>
        <p>我们采用"微型创业"式考核，通过模拟真实企业服务环境来选拔合格候选人。完整的考核流程如下：</p>
        <ol>
          <li><strong>了解动态</strong>：根据我们开源的海量信息和群内同步的实时进展，了解当前业务背景与需求。</li>
          <li><strong>制定课题</strong>：结合自己擅长和感兴趣的领域，自主制定课题。</li>
          <li><strong>申请与提交</strong>：向团队提出课题申请，完成后提交工作成果。</li>
          <li><strong>甲方评估</strong>：我们以甲方的视角评估你的工作成果——能否搞定客户、拿下订单、按时交付。</li>
        </ol>
        <p>你可以独立完成，也可以与其他求职者组队——我们推荐后者，企业服务从来不是一个人的战斗。</p>
        <p><a href="/intern/assessment">查看完整考核方式 →</a></p>
      </section>
    </>
  )
}

export default Intern
