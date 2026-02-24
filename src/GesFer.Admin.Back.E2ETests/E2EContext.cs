namespace GesFer.Admin.Back.E2ETests;

/// <summary>
/// Configuración del contexto E2E.
/// Variables de entorno: E2E_BASE_URL (ignorado si usa WebApplicationFactory), E2E_INTERNAL_SECRET, E2E_ADMIN_USER, E2E_ADMIN_PASSWORD.
/// </summary>
public static class E2EContext
{
    public static string InternalSecret { get; } =
        Environment.GetEnvironmentVariable("E2E_INTERNAL_SECRET")
        ?? "dev-internal-secret-change-in-production";

    public static string AdminUser { get; } =
        Environment.GetEnvironmentVariable("E2E_ADMIN_USER")
        ?? "admin";

    public static string AdminPassword { get; } =
        Environment.GetEnvironmentVariable("E2E_ADMIN_PASSWORD")
        ?? "admin123";
}
