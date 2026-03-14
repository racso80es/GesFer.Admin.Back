import { test, expect } from '@playwright/test';

const LOGIN_ENDPOINT = '/api/admin/auth/login';
const AUDIT_LOGS_ENDPOINT = '/api/admin/audit-logs';

interface AdminLoginRequest {
  Usuario: string;
  Contraseña: string;
}

interface AdminLoginResponse {
  token: string;
  username: string;
  role: string;
}

interface AuditLogDto {
  id: string;
  cursorId: string;
  username: string;
  action: string;
  httpMethod: string;
  path: string;
  additionalData: string | null;
  actionTimestamp: string;
}

interface AuditLogsPagedResponse {
  auditLogs: AuditLogDto[];
  totalCount: number;
  pageNumber: number;
  pageSize: number;
  totalPages: number;
}

test.describe('Login E2E', () => {
  test('rechaza logins incorrectos (401)', async ({ request }) => {
    const body: AdminLoginRequest = {
      Usuario: 'admin',
      Contraseña: 'wrong-password',
    };

    const response = await request.post(LOGIN_ENDPOINT, {
      data: body,
      headers: { 'Content-Type': 'application/json' },
    });

    expect(response.status()).toBe(401);

    const json = await response.json();
    expect(json).toHaveProperty('message');
    expect(json.message).toBeDefined();
  });

  test('rechaza credenciales vacías (400)', async ({ request }) => {
    const body: AdminLoginRequest = {
      Usuario: '',
      Contraseña: 'admin123',
    };

    const response = await request.post(LOGIN_ENDPOINT, {
      data: body,
      headers: { 'Content-Type': 'application/json' },
    });

    expect(response.status()).toBe(400);
  });

  test('acepta login correcto (admin/admin123) y devuelve token', async ({ request }) => {
    const body: AdminLoginRequest = {
      Usuario: 'admin',
      Contraseña: 'admin123',
    };

    const response = await request.post(LOGIN_ENDPOINT, {
      data: body,
      headers: { 'Content-Type': 'application/json' },
    });

    expect(response.status()).toBe(200);

    const result = (await response.json()) as AdminLoginResponse & { Token?: string };
    const token = result.token ?? result.Token;
    expect(token).toBeTruthy();
    expect(result.username ?? result.Username).toBe('admin');
    expect(result.role ?? result.Role).toBe('Admin');
  });

  test('genera registro de AuditLogs para login exitoso y fallido', async ({ request }) => {
    // 1. Login fallido para generar LoginFailed
    await request.post(LOGIN_ENDPOINT, {
      data: { Usuario: 'admin', Contraseña: 'wrong' } as AdminLoginRequest,
      headers: { 'Content-Type': 'application/json' },
    });

    // 2. Login exitoso para obtener token y generar LoginSuccess
    const loginResponse = await request.post(LOGIN_ENDPOINT, {
      data: { Usuario: 'admin', Contraseña: 'admin123' } as AdminLoginRequest,
      headers: { 'Content-Type': 'application/json' },
    });

    expect(loginResponse.status()).toBe(200);
    const loginResult = await loginResponse.json();
    const token = loginResult.token ?? loginResult.Token;
    expect(token).toBeTruthy();

    // 3. Obtener audit logs con el token JWT
    const authHeader = { Authorization: `Bearer ${token}` };

    // Verificar LoginFailed
    const failedResponse = await request.get(
      `${AUDIT_LOGS_ENDPOINT}?action=LoginFailed&username=admin`,
      { headers: authHeader }
    );
    expect(failedResponse.status()).toBe(200);

    const failedData = (await failedResponse.json()) as AuditLogsPagedResponse;
    const auditLogsFailed = failedData.auditLogs ?? failedData.AuditLogs ?? [];
    expect(auditLogsFailed.length).toBeGreaterThanOrEqual(1);
    const loginFailedEntry = auditLogsFailed.find(
      (l) => l.action === 'LoginFailed' && l.username === 'admin'
    );
    expect(loginFailedEntry).toBeDefined();
    expect(loginFailedEntry!.path).toBe('/api/admin/auth/login');
    expect(loginFailedEntry!.httpMethod).toBe('POST');

    // Verificar LoginSuccess
    const successResponse = await request.get(
      `${AUDIT_LOGS_ENDPOINT}?action=LoginSuccess&username=admin`,
      { headers: authHeader }
    );
    expect(successResponse.status()).toBe(200);

    const successData = (await successResponse.json()) as AuditLogsPagedResponse;
    const auditLogsSuccess = successData.auditLogs ?? successData.AuditLogs ?? [];
    expect(auditLogsSuccess.length).toBeGreaterThanOrEqual(1);
    const loginSuccessEntry = auditLogsSuccess.find(
      (l) => l.action === 'LoginSuccess' && l.username === 'admin'
    );
    expect(loginSuccessEntry).toBeDefined();
    expect(loginSuccessEntry!.path).toBe('/api/admin/auth/login');
    expect(loginSuccessEntry!.httpMethod).toBe('POST');
    expect(loginSuccessEntry!.cursorId).toBeTruthy();
  });
});
