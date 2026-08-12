import { useEffect, useState } from 'react'
import Dashboard from './pages/Dashboard'
import Join from './pages/Join'
import './App.css'

function App() {
  const [page, setPage] = useState(() => (window.location.hash === '#/join' ? 'join' : 'home'))

  useEffect(() => {
    const onHashChange = () => setPage(window.location.hash === '#/join' ? 'join' : 'home')
    window.addEventListener('hashchange', onHashChange)
    return () => window.removeEventListener('hashchange', onHashChange)
  }, [])

  return (
    <div className="page">
      <nav className="site-nav">
        <a className="site-brand" href="#/">量潮招聘</a>
        <div className="site-links">
          <a href="#/">招聘看板</a>
          <a href="#/join">加入我们</a>
        </div>
      </nav>
      {page === 'join' ? <Join /> : <Dashboard />}
    </div>
  )
}

export default App
