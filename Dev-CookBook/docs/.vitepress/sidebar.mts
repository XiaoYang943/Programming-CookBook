import { existsSync, readFileSync, readdirSync } from 'node:fs'
import { extname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import type { DefaultTheme } from 'vitepress'

const ignoredDirectories = new Set(['assets', 'node_modules'])
const ignoredMarkdownFiles = new Set(['AGENTS.md', 'README.md'])

function normalizeUrlPath(path: string): string {
  const normalizedPath = path.replaceAll('\\', '/').replace(/^\/+|\/+$/g, '')
  return normalizedPath ? `/${normalizedPath}/` : '/'
}

function normalizePageUrl(path: string): string {
  return `/${path.replaceAll('\\', '/').replace(/^\/+|\/+$/g, '')}`
}

function nameParts(name: string): { sectionNumber?: string, fallbackTitle: string } {
  const nameWithoutExtension = name.slice(0, name.length - extname(name).length)
  const match = nameWithoutExtension.match(/^(\d+(?:\.\d+)*)-(.+)$/)

  return match
    ? { sectionNumber: match[1], fallbackTitle: match[2] }
    : { fallbackTitle: nameWithoutExtension }
}

function pageTitle(markdownFile: string, sourceName: string): string {
  const { sectionNumber, fallbackTitle } = nameParts(sourceName)
  const markdown = readFileSync(markdownFile, 'utf8')
  const titlePattern = /^(?:#{1,6}\s+|\*\*)(?:(\d+(?:\.\d+)*)\s+)?(.+?)(?:\*\*)?\s*$/gm
  const titles = [...markdown.matchAll(titlePattern)]

  if (sectionNumber) {
    const matchingTitle = titles.find((match) => match[1] === sectionNumber)
    if (matchingTitle) {
      return matchingTitle[2].replace(/\*\*$/, '').trim()
    }
  }

  const firstTitle = titles[0]?.[2].replace(/\*\*$/, '').trim()
  return firstTitle || fallbackTitle
}

function compareNames(left: string, right: string): number {
  return left.localeCompare(right, 'zh-CN', { numeric: true })
}

function scanDirectory(directory: string, urlPath: string): DefaultTheme.SidebarItem[] {
  const entries = readdirSync(directory, { withFileTypes: true })
    .filter((entry) => !entry.name.startsWith('.'))
    .filter((entry) => !entry.isDirectory() || !ignoredDirectories.has(entry.name))
    .filter((entry) => !entry.isFile() || !ignoredMarkdownFiles.has(entry.name))
    .sort((left, right) => compareNames(left.name, right.name))

  return entries.flatMap((entry) => {
    const entryPath = join(directory, entry.name)

    if (entry.isDirectory()) {
      const indexFile = join(entryPath, 'index.md')
      const childUrlPath = [urlPath, entry.name].filter(Boolean).join('/')
      const children = scanDirectory(entryPath, childUrlPath)

      if (!existsSync(indexFile) && children.length === 0) {
        return []
      }

      const item: DefaultTheme.SidebarItem = {
        text: existsSync(indexFile)
          ? pageTitle(indexFile, entry.name)
          : nameParts(entry.name).fallbackTitle,
      }

      if (existsSync(indexFile)) {
        item.link = normalizeUrlPath(childUrlPath)
      }
      if (children.length > 0) {
        item.items = children
      }

      return [item]
    }

    if (entry.isFile() && entry.name.endsWith('.md') && entry.name !== 'index.md') {
      const pageName = entry.name.slice(0, -3)
      return [{
        text: pageTitle(entryPath, entry.name),
        link: normalizePageUrl([urlPath, pageName].filter(Boolean).join('/')),
      }]
    }

    return []
  })
}

export function generateSidebar(docsDirectory: URL): DefaultTheme.SidebarItem[] {
  const directory = fileURLToPath(docsDirectory)
  const homePage = join(directory, 'index.md')
  const sidebar: DefaultTheme.SidebarItem[] = []

  if (existsSync(homePage)) {
    sidebar.push({ text: '首页', link: '/' })
  }

  return [...sidebar, ...scanDirectory(directory, '')]
}
