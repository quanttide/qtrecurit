import { useState } from 'react'
import Home from './pages/Home'
import Intern from './pages/Intern'
import Parttime from './pages/Parttime'
import './App.css'

type View = 'home' | 'intern' | 'parttime'

const NAV: { key: View; label: string }[] = [
  { key: 'home', label: '首页' },
  { key: 'intern', label: '实习' },
  { key: 'parttime', label: '兼职' },
]

function App() {
  const [view, setView] = useState<View>('home')

  return (
    <div className="app">
      <nav className="site-nav">
        <div className="nav-inner">
          <a className="site-brand" href="#/">量潮招聘</a>
          <div className="site-links">
            {NAV.map((item) => (
              <a
                key={item.key}
                href={`#${item.key}`}
                className={view === item.key ? 'active' : ''}
                onClick={(e) => {
                  e.preventDefault()
                  setView(item.key)
                }}
              >
                {item.label}
              </a>
            ))}
          </div>
        </div>
      </nav>

      <header className="hero">
        <h1>你的创造力，值得被看见</h1>
      </header>

      <div className="content">
        {view === 'home' && <Home />}
        {view === 'intern' && <Intern />}
        {view === 'parttime' && <Parttime />}
      </div>
    </div>
  )
}

export default App
