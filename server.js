import { Hono } from 'hono'
import { serve } from '@hono/node-server'
import { readFile } from 'fs/promises'
import { join } from 'path'

const app = new Hono()

// Helper to serve HTML files
async function serveHtml(filePath) {
  const content = await readFile(filePath, 'utf-8')
  return new Response(content, {
    headers: { 'Content-Type': 'text/html; charset=utf-8' }
  })
}

// Root - English version
app.get('/', async (c) => {
  return serveHtml(join(process.cwd(), 'index.html'))
})

// Spanish version
app.get('/es', async (c) => {
  return serveHtml(join(process.cwd(), 'es', 'index.html'))
})

// French version
app.get('/fr', async (c) => {
  return serveHtml(join(process.cwd(), 'fr', 'index.html'))
})

// Hebrew version
app.get('/he', async (c) => {
  return serveHtml(join(process.cwd(), 'he', 'index.html'))
})

const port = process.env.PORT || 3000

console.log(`CSP Policy Builder running at http://localhost:${port}`)
console.log(`  English: http://localhost:${port}/`)
console.log(`  Spanish: http://localhost:${port}/es`)
console.log(`  French:  http://localhost:${port}/fr`)
console.log(`  Hebrew:  http://localhost:${port}/he`)

serve({ fetch: app.fetch, port })
