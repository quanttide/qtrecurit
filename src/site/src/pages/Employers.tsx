import { Link } from 'react-router-dom'
import { employers } from '../data/employers'
import { positions } from '../data/positions'

// 合作雇主列表页：/employers，各雇主卡片链接到独立详情页 /employers/:employerId
function Employers() {
  return (
    <>
      <section id="employers" className="join-section">
        <h2>合作雇主</h2>
        <p>
          除量潮自营岗位外，我们也为合作雇主提供招聘服务。点击查看各雇主的在招岗位与投递方式。
        </p>
      </section>

      <section className="join-section">
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
                  <span className="position-category">{count} 个在招岗位</span>
                </div>
                <p>{employer.intro}</p>
              </Link>
            )
          })}
        </div>
      </section>
    </>
  )
}

export default Employers
