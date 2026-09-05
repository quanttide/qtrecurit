import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom/vitest'
import { MemoryRouter } from 'react-router-dom'
import App, { Layout } from './App'

// 冒烟测试：每个路由必须渲染出内容。白屏（运行时 Hooks 错误、坏 import、渲染期崩溃）会在这里失败。
describe('页面渲染冒烟', () => {
  const routes = [
    { path: '/', expect: '量潮和它的朋友们的招聘门户' },
    { path: '/positions', expect: '在招岗位' },
    { path: '/positions?type=实习', expect: '在招岗位' },
    { path: '/positions?type=兼职', expect: '在招岗位' },
    { path: '/positions/application', expect: '简历要求' },
    { path: '/positions/assessment', expect: '以真实创业环境为中心的招聘考核机制' },
    { path: '/employers', expect: '合作雇主' },
    { path: '/employers/qttech', expect: '实习考核方式' },
    { path: '/employers/qttech', expect: '量潮科技' },
    { path: '/employers/dmu', expect: '大连医科大学课题组' },
  ]

  for (const { path, expect: text } of routes) {
    it(`渲染 ${path}`, () => {
      render(
        <MemoryRouter initialEntries={[path]}>
          <Layout />
        </MemoryRouter>,
      )
      expect(screen.getByText(text)).toBeInTheDocument()
    })
  }

  it('导航栏渲染完整', () => {
    render(
      <MemoryRouter initialEntries={['/']}>
        <Layout />
      </MemoryRouter>,
    )
    expect(screen.getByText('首页')).toBeInTheDocument()
    expect(screen.getByText('岗位')).toBeInTheDocument()
    expect(screen.getByText('雇主')).toBeInTheDocument()
  })

  // BrowserRouter 整树挂载：验证 App 出口本身可渲染（不嵌套 Router）
  it('App 可完整挂载', () => {
    render(<App />)
    expect(screen.getByText('量潮招聘')).toBeInTheDocument()
  })
})
