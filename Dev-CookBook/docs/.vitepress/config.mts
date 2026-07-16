import { defineConfig } from 'vitepress'
import { generateSidebar } from './sidebar.mts'

const sidebar = generateSidebar(new URL('../', import.meta.url))

export default defineConfig({
  title: 'Dev-CookBook',
  description: '个人开发知识库',
  lang: 'zh-CN',
  cleanUrls: true,
  lastUpdated: true,
  markdown: {
    math: true
  },
  themeConfig: {
    logo: '/logo.svg',
    nav: [
      { text: '首页', link: '/' },
      { text: '地理空间数据库', link: '/地理空间数据库/' },
      { text: '数据结构与算法', link: '/数据结构与算法/' },
    ],
    sidebar,
    search: {
      provider: 'local'
    },
    outline: {
      level: [2, 5],
      label: '页面导航'
    },
    docFooter: {
      prev: '上一页',
      next: '下一页'
    },
    lastUpdated: {
      text: '最后更新'
    }
  }
})
