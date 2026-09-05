import { BrowserRouter, Routes, Route, NavLink, useLocation } from 'react-router-dom'
import Home from './pages/Home'
import Positions from './pages/Positions'
import Assessment from './pages/Assessment'
import Application from './pages/Application'
import Employers from './pages/Employers'
import Employer from './pages/Employer'
import './App.css'

// BrowserRouter（History 路由）：/positions、/employers 及其子路由独立 URL。
// 静态托管子路由回落：OSS 桶 error_document 已改为 index.html（见 manifests/terraform/site.tf）。
function App() {
  return (
    <BrowserRouter>
      <Layout />
    </BrowserRouter>
  )
}

// 布局与路由单独导出：测试中用 MemoryRouter 包裹，避免 Router 嵌套
export function Layout() {
  const location = useLocation()
  return (
    <div className="app">
      <nav className="site-nav">
        <div className="nav-inner">
          <a className="site-brand" href="/">量潮招聘</a>
          <div className="site-links">
            <NavLink to="/" end className={({ isActive }) => (isActive ? 'active' : '')}>
              首页
            </NavLink>
            <NavLink
              to="/positions"
              className={({ isActive }) =>
                isActive || location.pathname.startsWith('/positions/') ? 'active' : ''
              }
            >
              岗位
            </NavLink>
            <NavLink
              to="/employers"
              className={({ isActive }) => (isActive ? 'active' : '')}
            >
              雇主
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
          <Route path="/positions" element={<Positions />} />
          <Route path="/positions/assessment" element={<Assessment />} />
          <Route path="/positions/application" element={<Application />} />
          <Route path="/employers" element={<Employers />} />
          <Route path="/employers/:employerId" element={<Employer />} />
        </Routes>
      </div>
    </div>
  )
}

export default App
