import Join from './pages/Join'
import './App.css'

function App() {
  return (
    <div className="app">
      <nav className="site-nav">
        <div className="nav-inner">
          <a className="site-brand" href="#/">量潮招聘</a>
          <div className="site-links">
            <a href="#company">公司简介</a>
            <a href="#positions">招聘岗位</a>
            <a href="#apply">报名方式</a>
          </div>
        </div>
      </nav>
      <Join />
    </div>
  )
}

export default App
