using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Domain.Services;
using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Infrastructure.Services;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Pomelo.EntityFrameworkCore.MySql;

namespace GesFer.Admin.Back.Infrastructure;

/// <summary>
/// Configuración de inyección de dependencias de la capa Infrastructure (DbContext, servicios de datos y auth).
/// </summary>
public static class DependencyInjection
{
    /// <summary>
    /// Registra servicios de infraestructura: DbContext, Auth, JWT, AuditLog, Seeder, GuidGenerator, Sanitizer.
    /// </summary>
    /// <param name="isDevelopment">Si es true, habilita SensitiveDataLogging y DetailedErrors en DbContext.</param>
    public static IServiceCollection AddInfrastructureServices(
        this IServiceCollection services,
        IConfiguration configuration,
        bool isDevelopment = false)
    {
        var connectionString = configuration.GetConnectionString("DefaultConnection")
            ?? throw new InvalidOperationException("Connection string 'DefaultConnection' not found.");

        services.AddDbContext<AdminDbContext>((serviceProvider, options) =>
        {
            options.UseMySql(
                connectionString,
                new MySqlServerVersion(new Version(8, 0, 0)),
                mysqlOptions =>
                {
                    mysqlOptions.EnableStringComparisonTranslations();
                    mysqlOptions.EnableRetryOnFailure(
                        maxRetryCount: 5,
                        maxRetryDelay: TimeSpan.FromSeconds(30),
                        errorNumbersToAdd: null);
                    mysqlOptions.MigrationsHistoryTable("__EFMigrationsHistory_Admin");
                });

            if (isDevelopment)
            {
                options.EnableSensitiveDataLogging();
                options.EnableDetailedErrors();
            }
        });

        services.AddScoped<IApplicationDbContext>(provider => provider.GetRequiredService<AdminDbContext>());
        services.AddScoped<IAdminAuthService, AdminAuthService>();
        services.AddScoped<IAdminJwtService, AdminJwtService>();
        services.AddScoped<IAuditLogService, AuditLogService>();
        services.AddScoped<AdminJsonDataSeeder>();
        services.AddSingleton<ISequentialGuidGenerator, MySqlSequentialGuidGenerator>();
        services.AddSingleton<ISensitiveDataSanitizer, SensitiveDataSanitizer>();

        return services;
    }
}
