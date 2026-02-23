using System.Reflection;
using MediatR;
using Microsoft.Extensions.DependencyInjection;

namespace GesFer.Admin.Back.Application;

/// <summary>
/// Configuración de inyección de dependencias de la capa Application (sin referencias a Infrastructure).
/// </summary>
public static class DependencyInjection
{
    /// <summary>
    /// Registra servicios de la capa Application: MediatR, validators, behaviors.
    /// </summary>
    public static IServiceCollection AddApplicationServices(this IServiceCollection services)
    {
        services.AddMediatR(cfg => cfg.RegisterServicesFromAssembly(Assembly.GetExecutingAssembly()));
        return services;
    }
}
