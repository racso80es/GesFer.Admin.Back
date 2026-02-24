namespace GesFer.Admin.Back.E2ETests;

/// <summary>
/// Configuración del contexto E2E: API real en local con infra y datos preparados.
/// Variables de entorno: E2E_BASE_URL, E2E_INTERNAL_SECRET, E2E_ADMIN_USER, E2E_ADMIN_PASSWORD.
/// </summary>
public static class E2EContext
{
    public static string BaseUrl { get; } =
        Environment.GetEnvironmentVariable("E2E_BASE_URL")?.TrimEnd('/')
        ?? "http://localhost:5010";

    public static string InternalSecret { get; } =
        Environment.GetEnvironmentVariable("E2E_INTERNAL_SECRET")
        ?? "dev-internal-secret-change-in-production";

    public static string AdminUser { get; } =
        Environment.GetEnvironmentVariable("E2E_ADMIN_USER")
        ?? "admin";

    public static string AdminPassword { get; } =
        Environment.GetEnvironmentVariable("E2E_ADMIN_PASSWORD")
        ?? "admin123";

    public static HttpClient CreateClient(bool withInternalSecret = false)
    {
        var client = new HttpClient { BaseAddress = new Uri(BaseUrl) };
        client.DefaultRequestHeaders.Add("Accept", "application/json");
        if (withInternalSecret)
            client.DefaultRequestHeaders.Add("X-Internal-Secret", InternalSecret);
        return client;
    }
}
