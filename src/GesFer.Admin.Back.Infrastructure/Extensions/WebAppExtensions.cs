using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Infrastructure.Services;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Infrastructure;

/// <summary>
/// Extensiones para el host web (migraciones y seeds). Permite que Api/Program no dependa de tipos concretos de Infrastructure en el cuerpo.
/// </summary>
public static class WebAppExtensions
{
    /// <summary>
    /// Ejecuta migraciones y seeds de Admin (MigrateAsync + SeedAllAsync). Llamar desde Program: await app.Services.RunMigrationsAndSeedsAsync().
    /// </summary>
    public static async Task RunMigrationsAndSeedsAsync(this IServiceProvider serviceProvider)
    {
        using var scope = serviceProvider.CreateScope();
        var services = scope.ServiceProvider;
        var logger = services.GetRequiredService<ILoggerFactory>().CreateLogger("GesFer.Admin.Back.Infrastructure.WebAppExtensions");
        try
        {
            var context = services.GetRequiredService<AdminDbContext>();
            await context.Database.MigrateAsync();
            var seeder = services.GetRequiredService<AdminJsonDataSeeder>();
            await seeder.SeedAllAsync();
        }
        catch (Exception ex)
        {
            logger.LogError(ex, "Error en migraciones o seeds: {Message}", ex.Message);
            throw;
        }
    }

    /// <summary>
    /// Ejecuta migraciones, seeds y sale (modo RUN_SEEDS_ONLY). Para scripts/tools.
    /// </summary>
    public static async Task RunMigrationsAndSeedsThenExitAsync(this IServiceProvider serviceProvider)
    {
        using var scope = serviceProvider.CreateScope();
        var services = scope.ServiceProvider;
        var logger = services.GetRequiredService<ILoggerFactory>().CreateLogger("GesFer.Admin.Back.Infrastructure.WebAppExtensions");
        try
        {
            var context = services.GetRequiredService<AdminDbContext>();
            await context.Database.MigrateAsync();
            var seeder = services.GetRequiredService<AdminJsonDataSeeder>();
            await seeder.SeedAllAsync();
            Environment.Exit(0);
        }
        catch (Exception ex)
        {
            logger.LogError(ex, "Error en migraciones o seeds (RUN_SEEDS_ONLY): {Message}", ex.Message);
            throw;
        }
    }
}
