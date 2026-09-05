import { Link } from 'react-router-dom'
import { employers } from '../data/employers'
import { positions, type Employment } from '../data/positions'

// 首页：招聘门户定位——量潮和它的朋友们的招聘入口，按就业类型与雇主两个维度导流
function Home() {
  const types: { label: Employment; desc: string }[] = [
    { label: '全职', desc: '合作雇主发布的中长期岗位' },
    { label: '实习', desc: '「微型创业」式考核，组队参与' },
    { label: '兼职', desc: '按成交分成，时间灵活' },
  ]

  return (
    <>
      <section id="portal" className="join-section">
        <h2>量潮和它的朋友们的招聘门户</h2>
        <p>
          这里汇集量潮科技自营岗位与合作雇主的在招机会。你可以按就业类型浏览岗位，也可以直接进入感兴趣的雇主主页了解详情与应聘方式。
        </p>
      </section>

      <section className="join-section">
        <h2>按类型找岗位</h2>
        <div className="position-grid">
          {types.map(({ label, desc }) => {
            const count = positions.filter((p) => p.employment === label).length
            return (
              <Link key={label} to={`/positions?type=${label}`} className="position-card">
                <div className="position-head">
                  <h3>{label}</h3>
                  <span className="position-category">{count} 个在招</span>
                </div>
                <p>{desc}</p>
              </Link>
            )
          })}
        </div>
      </section>

      <section className="join-section">
        <h2>按雇主找岗位</h2>
        <div className="position-grid">
          {employers.map((employer) => {
            const count = positions.filter((p) => p.employerId === employer.id).length
            return (
              <Link
                key={employer.id}
                to={`/employers/${employer.id}`}
                className="position-card"
              >
                <div className="position-head">
                  <h3>{employer.name}</h3>
                  <span className="position-category">{count} 个在招</span>
                </div>
                <p>{employer.intro}</p>
              </Link>
            )
          })}
        </div>
        <p>
          <Link to="/employers">查看全部雇主 →</Link>
        </p>
      </section>
    </>
  )
}

export default Home
