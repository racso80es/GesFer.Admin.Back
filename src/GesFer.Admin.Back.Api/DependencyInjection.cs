
using GesFer.Admin.Back.Infrastructure.Services;
using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Domain.Services;
using GesFer.Admin.Back.Application.Common.Interfaces;
using Microsoft.EntityFrameworkCore;
using Pomelo.EntityFrameworkCore.MySql;

namespace GesFer.Admin.Back.Api;

/// <summary>
/// Configuración de inyección de dependencias para Admin API
/// </summary>
public static class DependencyInjection
{
    /// <summary>
    /// Registra todos los servicios de la aplicación Admin
    /// </summary>
    public static IServiceCollection AddApplicationServices(this IServiceCollection services, IConfiguration configuration, IWebHostEnvironment? environment = null)
    {
        // Configurar DbContext - Usar la misma cadena de conexión que Product
        var connectionString = configuration.GetConnectionString("DefaultConnection") 
            ?? "Server=localhost;Port=3306;Database=ScrapDb;User=scrapuser;Password=scrappassword;CharSet=utf8mb4;AllowUserVariables=True;AllowLoadLocalInfile=True;";

        var isDevelopment = environment?.IsDevelopment() ?? false;

        // 1. Contexto de Admin (Escritura de Logs, Audit, AdminUsers)
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
                    // Usar tabla de historial de migraciones separada
                    mysqlOptions.MigrationsHistoryTable("__EFMigrationsHistory_Admin");
                });

            if (isDevelopment)
            {
                options.EnableSensitiveDataLogging();
                options.EnableDetailedErrors();
            }
        });

        // Registrar IApplicationDbContext apuntando a AdminDbContext
        services.AddScoped<IApplicationDbContext>(provider => provider.GetRequiredService<AdminDbContext>());

        // Servicios de infraestructura Admin
        services.AddScoped<IAdminAuthService, AdminAuthService>();
        services.AddScoped<IAdminJwtService, AdminJwtService>();
        services.AddScoped<IAuditLogService, AuditLogService>();
        services.AddScoped<AdminJsonDataSeeder>();

        // Servicios Shared
        services.AddSingleton<ISequentialGuidGenerator, MySqlSequentialGuidGenerator>();
        services.AddSingleton<ISensitiveDataSanitizer, SensitiveDataSanitizer>();

        // Registrar MediatR para manejar comandos/queries
        // Escanear el ensamblado de Application
        services.AddMediatR(cfg => cfg.RegisterServicesFromAssembly(typeof(GesFer.Admin.Back.Application.DTOs.Company.CompanyDto).Assembly));

        return services;
    }
}
