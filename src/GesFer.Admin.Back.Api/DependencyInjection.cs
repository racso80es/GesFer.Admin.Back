using GesFer.Admin.Back.Application;
using GesFer.Admin.Back.Infrastructure;

namespace GesFer.Admin.Back.Api;

/// <summary>
/// Punto de composición de inyección de dependencias para Admin API.
/// Delega en Application e Infrastructure; sin referencias a tipos concretos de Infrastructure.Data ni Infrastructure.Services.
/// </summary>
public static class DependencyInjection
{
    /// <summary>
    /// Registra todos los servicios de la aplicación Admin (Application + Infrastructure).
    /// </summary>
    public static IServiceCollection AddApplicationServices(
        this IServiceCollection services,
        IConfiguration configuration,
        IWebHostEnvironment? environment = null)
    {
        services.AddApplicationServices();
        services.AddInfrastructureServices(configuration, environment?.IsDevelopment() ?? false);
        return services;
    }
}
