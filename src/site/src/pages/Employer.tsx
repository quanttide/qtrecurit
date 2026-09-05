import { Link, useParams } from 'react-router-dom'
import { employers } from '../data/employers'
import { positions } from '../data/positions'

// 单个合作雇主页：/employers/:employerId，链接可直接转发给雇主自行传播。
// 报名方式、考核流程等雇主相关信息集中在此页，岗位一览页只做岗位浏览。
function EmployerPage() {
  const { employerId } = useParams()
  const employer = employers.find((e) => e.id === employerId)

  if (!employer) {
    return (
      <section className="join-section">
        <h2>未找到该合作雇主</h2>
        <p>
          <Link to="/employers">← 返回合作雇主列表</Link>
        </p>
      </section>
    )
  }

  const employerPositions = positions.filter((p) => p.employerId === employer.id)

  return (
    <>
      <section className="join-section">
        <p>
          <Link to="/employers">← 合作雇主</Link>
        </p>
        <h2>{employer.name}</h2>
        <p>{employer.intro}</p>
      </section>

      {/* 量潮自营的公司与业务介绍；合作雇主的信息由 intro 承载 */}
      {employer.id === 'qttech' && (
        <section className="join-section">
          <h2>主营业务</h2>
          <ul>
            <li>量潮数据：为高校与科技企业提供高质量、高性价比的数据处理定制服务。</li>
            <li>量潮课堂：为高校与企业提供大数据技术培训与课程，负责浙理工大数据微专业。</li>
            <li>量潮咨询：帮助中小企业转型 AI 原生组织。</li>
          </ul>
        </section>
      )}

      <section className="join-section">
        <h2>在招岗位</h2>
        <div className="position-grid">
          {employerPositions.map((p) => (
            <div key={p.name} className="position-card">
              <div className="position-head">
                <h3>{p.name}</h3>
                <span className="position-category">{p.category}</span>
              </div>
              <p>{p.duty}</p>
              {p.salary && (
                <p>
                  <strong>薪酬待遇</strong>：{p.salary}
                </p>
              )}
              <p className="position-tags">
                <span className="position-tag">{p.employment}</span>
              </p>
            </div>
          ))}
        </div>
      </section>

      <section className="join-section" id="apply">
        <h2>应聘方式</h2>
        <p>
          将个人简历（含基本信息、教育背景、科研/工作经历、发表论文等）发送至{' '}
          <strong>{employer.applyEmail}</strong>，邮件主题注明
          「{employer.subjectFormat}」。初审通过者将通知面试。
        </p>
        {employer.id === 'qttech' && (
          <p>
            <Link to="/positions/application">查看简历和求职信要求 →</Link>
          </p>
        )}
      </section>

      {/* 量潮自营的实习考核与兼职流程；后续多雇主时改为按雇主配置 */}
      {employer.id === 'qttech' && (
        <>
          <section className="join-section">
            <h2>实习考核方式</h2>
            <p>
              我们采用「微型创业」式考核，通过模拟真实企业服务环境来选拔合格候选人：填写我们的问卷后，我们会邀请你进群，整个考核围绕一个由你自己提出的课题展开，最长不超过一个月。流程如下：
            </p>
            <ol>
              <li>
                <strong>了解动态</strong>
                ：根据我们开源的海量信息和群内同步的实时进展，了解当前业务背景与需求。
              </li>
              <li>
                <strong>制定课题</strong>
                ：结合自己擅长和感兴趣的领域，自主制定课题；众包平台上的任务就是示例课题。
              </li>
              <li>
                <strong>申请课题</strong>
                ：通过 hr 邮箱提交课题方案，可申请 1 次、最多迭代 3 次。
              </li>
              <li>
                <strong>提交成果</strong>
                ：按方案推进，完成后提交工作成果。
              </li>
              <li>
                <strong>甲方评估</strong>
                ：以甲方视角评估协作表现，通过后进入入职环节。
              </li>
            </ol>
            <p>
              你可以独立完成，也可以与其他求职者组队——我们推荐后者，企业服务从来不是一个人的战斗。
            </p>
            <p>
              <Link to="/positions/assessment">查看完整考核方式 →</Link>
            </p>
          </section>

          <section className="join-section">
            <h2>兼职提成与结算</h2>
            <ul>
              <li>提成：客户付款金额的 <strong>20%</strong>。</li>
              <li>
                分帐：<strong>客户打款后</strong>
                ，量潮按 20% 与销售分帐。
              </li>
            </ul>
            <h3>兼职流程</h3>
            <ol>
              <li>
                联系：将个人或团队介绍发送至 <strong>hr@quanttide.com</strong>
                ，说明资源与获客方式。
              </li>
              <li>洽谈：明确可售的定制服务范围（量潮数据/课堂/咨询）与分成比例（20%）。</li>
              <li>获客：为对应定制服务拓展客户、获取线索或促成成交。</li>
              <li>结算：客户打款后，量潮按 20% 与销售分帐。</li>
            </ol>
          </section>
        </>
      )}
    </>
  )
}

export default EmployerPage
