import { BrowserRouter, Routes, Route, NavLink } from 'react-router-dom'
import Home from './pages/Home'
import Intern from './pages/Intern'
import Parttime from './pages/Parttime'
import './App.css'

// BrowserRouter（History 路由）：/intern、/parttime 独立 URL。
// 静态托管子路由回落：OSS 桶 error_document 已改为 index.html（见 manifests/terraform/site.tf）。
function App() {
  return (
    <BrowserRouter>
      <div className="app">
        <nav className="site-nav">
          <div className="nav-inner">
            <a className="site-brand" href="/">量潮招聘</a>
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
    </BrowserRouter>
  )
}

export default App
