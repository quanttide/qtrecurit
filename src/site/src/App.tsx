import './App.css'

const positions = [
  { name: '数据工程师', count: 63 },
  { name: '新媒体运营', count: 48 },
  { name: '商务经理', count: 18 },
  { name: '项目经理', count: 13 },
  { name: '人事经理', count: 10 },
  { name: '课程助教', count: 4 },
  { name: '销售经理', count: 4 },
  { name: '法务实习生', count: 3 },
  { name: '产品经理', count: 2 },
  { name: '全栈工程师', count: 2 },
]

const maxCount = Math.max(...positions.map(p => p.count))

function App() {
  return (
    <div className="page">
      <header>
        <h1>量潮招聘</h1>
        <p className="period">2026年6月上半月 · 工作报告</p>
      </header>

      <section className="summary">
        <div className="card">
          <span className="num">206</span>
          <span className="label">总投递</span>
        </div>
        <div className="card accent">
          <span className="num">167</span>
          <span className="label">有效投递</span>
        </div>
        <div className="card muted">
          <span className="num">39</span>
          <span className="label">未识别</span>
        </div>
      </section>

      <section className="positions">
        <h2>岗位分布</h2>
        <div className="bar-chart">
          {positions.map(p => (
            <div key={p.name} className="bar-row">
              <span className="bar-label">{p.name}</span>
              <div className="bar-track">
                <div
                  className="bar-fill"
                  style={{ width: `${(p.count / maxCount) * 100}%` }}
                />
              </div>
              <span className="bar-count">{p.count}</span>
            </div>
          ))}
        </div>
      </section>

      <section className="notes">
        <h2>招聘动态</h2>
        <ul>
          <li>投递高峰集中在 6 月 13-15 日，三日内共计 97 封，占全月 47%</li>
          <li>数据工程师、新媒体运营、商务经理等岗位已饱和</li>
          <li>销售经理为当前最高优先级岗位</li>
          <li>产品经理、全栈工程师暂不在本月重点，预计 7-8 月启动</li>
          <li>拟新增高优先级岗位：执行助理（技术/管理）、咨询助理（技术/管理）</li>
        </ul>
      </section>
    </div>
  )
}

export default App
