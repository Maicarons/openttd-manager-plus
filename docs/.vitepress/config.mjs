import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'zh-CN',
  title: 'OpenTTD Manager Plus',
  description: '跨平台开源 OpenTTD 版本管理器与启动器',
  lastUpdated: true,
  cleanUrls: true,

  head: [
    ['link', { rel: 'icon', href: '/favicon.svg' }],
    ['meta', { name: 'theme-color', content: '#4a9e4a' }],
  ],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: '首页', link: '/' },
      { text: '项目书', link: '/project-overview' },
      { text: '开发计划', link: '/development-plan' },
      { text: 'GitHub', link: 'https://github.com/user/openttd-manager-plus' },
    ],

    sidebar: [
      {
        text: '项目概览',
        items: [
          { text: '项目介绍', link: '/project-overview' },
          { text: '技术栈', link: '/technology-stack' },
          { text: '架构设计', link: '/architecture' },
        ],
      },
      {
        text: '功能规格',
        items: [
          { text: '功能总览', link: '/features' },
          { text: '版本管理', link: '/features/version-management' },
          { text: '下载引擎', link: '/features/download-engine' },
          { text: '配置管理', link: '/features/config-management' },
          { text: '模组管理', link: '/features/mod-management' },
          { text: '启动器', link: '/features/launcher' },
        ],
      },
      {
        text: '平台支持',
        items: [
          { text: '跨平台概览', link: '/platform-support' },
          { text: '桌面端', link: '/platform/desktop' },
          { text: '移动端 (Android)', link: '/platform/mobile' },
        ],
      },
      {
        text: '开发',
        items: [
          { text: '开发计划', link: '/development-plan' },
          { text: 'UI/UX 设计', link: '/ui-ux-design' },
          { text: '构建与发布', link: '/build-release' },
          { text: '贡献指南', link: '/contributing' },
          { text: 'Phase 1 完成报告', link: '/phase1-complete' },
          { text: 'Phase 2 完成报告', link: '/phase2-complete' },
          { text: 'Phase 3 完成报告', link: '/phase3-complete' },
          { text: 'Phase 4 完成报告', link: '/phase4-complete' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/user/openttd-manager-plus' },
    ],

    footer: {
      message: '以 AGPL-3.0 许可证发布',
      copyright: 'Copyright © 2026 OpenTTD Manager Plus Contributors',
    },

    editLink: {
      pattern: 'https://github.com/user/openttd-manager-plus/edit/main/docs/:path',
      text: '在 GitHub 上编辑此页',
    },
  },
})