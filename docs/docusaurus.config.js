/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'ОСАГО API',
  tagline: 'Документация сервиса расчёта стоимости ОСАГО',
  url: 'http://localhost:3000',
  baseUrl: '/',
  organizationName: 'VladislavOshkanov',
  projectName: 'openapi',
  onBrokenLinks: 'throw',
  i18n: {
    defaultLocale: 'ru',
    locales: ['ru'],
  },
  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          routeBasePath: '/docs',
        },
        blog: false,
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
  themeConfig: {
    navbar: {
      title: 'ОСАГО API',
      items: [
        { type: 'docSidebar', sidebarId: 'docsSidebar', position: 'left', label: 'Документация' },
        { href: 'https://github.com/VladislavOshkanov/calc-api', label: 'GitHub', position: 'right' },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Документация',
          items: [
            { label: 'Введение', to: '/docs/intro' },
            { label: 'API', to: '/docs/api-reference' },
            { label: 'Модели', to: '/docs/models' },
          ],
        },
        {
          title: 'Проект',
          items: [
            { label: 'GitHub', href: 'https://github.com/VladislavOshkanov/calc-api' },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} ОСАГО API`,
    },
    prism: {
      theme: undefined,
      darkTheme: undefined,
    },
  },
};

module.exports = config;
