import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';

const highlights = [
  {
    title: 'Публичный калькулятор',
    text: 'Пользовательский интерфейс на главной странице рассчитывает стоимость ОСАГО по выбранным коэффициентам.',
  },
  {
    title: 'Админ-панель',
    text: 'Отдельный интерфейс /admin управляет справочниками и защищён Bearer-токеном.',
  },
  {
    title: 'MongoDB-хранилище',
    text: 'Справочники коэффициентов и базовый тариф хранятся в MongoDB и загружаются сидерами.',
  },
];

export default function Home() {
  return (
    <Layout
      title="ОСАГО API"
      description="Документация сервиса расчёта стоимости ОСАГО"
    >
      <main style={{ padding: '4rem 1.5rem', maxWidth: '1100px', margin: '0 auto' }}>
        <section style={{ maxWidth: '720px', marginBottom: '3rem' }}>
          <h1>ОСАГО API</h1>
          <p style={{ fontSize: '1.125rem', lineHeight: 1.7 }}>
            REST API и веб-интерфейс для расчёта стоимости ОСАГО на основе базовой ставки и
            коэффициентов. Здесь описаны архитектура, модели, эндпоинты и сценарии запуска.
          </p>
          <div style={{ display: 'flex', gap: '1rem', flexWrap: 'wrap', marginTop: '1.5rem' }}>
            <Link className="button button--primary" to="/docs/intro">
              Открыть документацию
            </Link>
            <Link className="button button--secondary" to="/docs/api-reference">
              Перейти к API
            </Link>
          </div>
        </section>

        <section
          style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))',
            gap: '1rem',
          }}
        >
          {highlights.map((item) => (
            <article
              key={item.title}
              style={{
                border: '1px solid var(--ifm-toc-border-color)',
                borderRadius: '12px',
                padding: '1.25rem',
                background: 'var(--ifm-card-background-color)',
              }}
            >
              <h2 style={{ fontSize: '1.1rem' }}>{item.title}</h2>
              <p style={{ marginBottom: 0, lineHeight: 1.6 }}>{item.text}</p>
            </article>
          ))}
        </section>
      </main>
    </Layout>
  );
}
