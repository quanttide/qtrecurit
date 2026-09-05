import { Link, useSearchParams } from 'react-router-dom'
import { employers } from '../data/employers'
import { positions, type Employment } from '../data/positions'

const employmentTypes: ('全部' | Employment)[] = ['全部', '全职', '实习', '兼职']

// 岗位一览：/positions?type=，只做岗位浏览；报名方式、考核流程等雇主相关信息在雇主页
function Positions() {
  const [searchParams, setSearchParams] = useSearchParams()
  const type = (searchParams.get('type') as Employment | null) ?? '全部'

  const filtered = type === '全部' ? positions : positions.filter((p) => p.employment === type)

  // 按雇主分组；雇主顺序跟随 employers.ts 定义（量潮在前）
  const groups = employers
    .map((employer) => ({
      employer,
      items: filtered.filter((p) => p.employerId === employer.id),
    }))
    .filter((g) => g.items.length > 0)

  return (
    <>
      <section className="join-section">
        <h2>在招岗位</h2>
        <div className="filter-tabs">
          {employmentTypes.map((t) => (
            <button
              key={t}
              className={t === type ? 'filter-tab active' : 'filter-tab'}
              onClick={() => setSearchParams(t === '全部' ? {} : { type: t })}
            >
              {t}
            </button>
          ))}
        </div>
      </section>

      {groups.map(({ employer, items }) => (
        <section key={employer.id} className="join-section">
          <h2>
            <Link to={`/employers/${employer.id}`}>{employer.name}</Link>
          </h2>
          <div className="position-grid">
            {items.map((p) => (
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
                  <Link
                    className="position-apply"
                    to={`/employers/${employer.id}#apply`}
                  >
                    申请方式 →
                  </Link>
                </p>
              </div>
            ))}
          </div>
        </section>
      ))}

      {groups.length === 0 && (
        <section className="join-section">
          <p>暂无该类型的在招岗位。</p>
        </section>
      )}
    </>
  )
}

export default Positions
