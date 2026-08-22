import { HashRouter, Routes, Route, NavLink } from 'react-router-dom'
import Home from './pages/Home'
import Intern from './pages/Intern'
import Parttime from './pages/Parttime'
import './App.css'

// 用 HashRouter：静态托管 + 无 SPA 回退时，/#/intern、/#/parttime 可独立访问且刷新不 404。
function App() {
  return (
    <HashRouter>
      <div className="app">
        <nav className="site-nav">
          <div className="nav-inner">
            <a className="site-brand" href="#/">量潮招聘</a>
            <div className="site-links">
              <NavLink to="/" end className={({ isActive }) => (isActive ? 'active' : '')}>
                首页
              </NavLink>
              <NavLink to="/intern" className={({ isActive }) => (isActive ? 'active' : '')}>
                实习
              </NavLink>
              <NavLink to="/parttime" className={({ isActive }) => (isActive ? 'active' : '')}>
                兼职
              </NavLink>
            </div>
          </div>
        </nav>

        <header className="hero">
          <h1>你的创造力，值得被看见</h1>
        </header>

        <div className="content">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/intern" element={<Intern />} />
            <Route path="/parttime" element={<Parttime />} />
          </Routes>
        </div>
      </div>
    </HashRouter>
  )
}

export default App
