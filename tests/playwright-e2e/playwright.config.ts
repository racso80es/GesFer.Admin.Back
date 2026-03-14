import { defineConfig, devices } from '@playwright/test';

/**
 * Configuración Playwright para tests E2E del endpoint de login.
 * Requiere la API en ejecución (docker-compose o dotnet run).
 */
export default defineConfig({
  testDir: './tests',
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: [['html', { outputFolder: 'playwright-report' }]],
  use: {
    baseURL: process.env.BASE_URL ?? 'http://localhost:5010',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'login-api',
      testMatch: /login\.spec\.ts/,
    },
  ],
  timeout: 15000,
});
