import { positions } from '../data/positions'

const sales = positions.find((p) => p.employment === '兼职')

function Parttime() {
  return (
    <>
      <section className="join-section">
        <h2>兼职</h2>
        {sales && (
          <div className="position-card">
            <div className="position-head">
              <h3>{sales.name}</h3>
              <span className="position-category">{sales.category}</span>
            </div>
            <p>{sales.duty}</p>
          </div>
        )}
      </section>

      <section className="join-section">
        <h2>提成与结算</h2>
        <ul>
          <li>提成：客户付款金额的 <strong>20%</strong>。</li>
          <li>分帐：<strong>客户打款后</strong>，量潮按 20% 与销售分帐。</li>
        </ul>
      </section>

      <section className="join-section">
        <h2>兼职流程</h2>
        <ol>
          <li>联系：将个人或团队介绍发送至 <strong>hr@quanttide.com</strong>，说明资源与获客方式。</li>
          <li>洽谈：明确可售的定制服务范围（量潮数据/课堂/咨询）与分成比例（20%）。</li>
          <li>获客：为对应定制服务拓展客户、获取线索或促成成交。</li>
          <li>结算：客户打款后，量潮按 20% 与销售分帐。</li>
        </ol>
      </section>
    </>
  )
}

export default Parttime
