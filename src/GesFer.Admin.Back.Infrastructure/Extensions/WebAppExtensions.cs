using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Infrastructure.Services;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;

namespace GesFer.Admin.Back.Infrastructure;

/// <summary>
/// Extensiones para el host web (migraciones y seeds). Permite que Api/Program no dependa de tipos concretos de Infrastructure en el cuerpo.
/// </summary>
public static class WebAppExtensions
{
    /// <summary>
    /// Ejecuta migraciones y seeds de Admin (DbContext y AdminJsonDataSeeder). Llamar desde Program: await app.Services.RunMigrationsAndSeedsAsync().
    /// </summary>
    public static async Task RunMigrationsAndSeedsAsync(this IServiceProvider serviceProvider)
    {
        using var scope = serviceProvider.CreateScope();
        var services = scope.ServiceProvider;
        try
        {
            var context = services.GetRequiredService<AdminDbContext>();
            var seeder = services.GetRequiredService<AdminJsonDataSeeder>();
            await seeder.SeedCompaniesAsync();
            await seeder.SeedAdminUsersAsync();
        }
        catch (Exception)
        {
            // El llamador (Program) puede registrar el error si dispone de logger.
        }
    }

    /// <summary>
    /// Ejecuta migraciones, seeds y sale (modo RUN_SEEDS_ONLY). Para scripts/tools.
    /// </summary>
    public static async Task RunMigrationsAndSeedsThenExitAsync(this IServiceProvider serviceProvider)
    {
        using var scope = serviceProvider.CreateScope();
        var services = scope.ServiceProvider;
        var context = services.GetRequiredService<AdminDbContext>();
        await context.Database.MigrateAsync();
        var seeder = services.GetRequiredService<AdminJsonDataSeeder>();
        await seeder.SeedCompaniesAsync();
        await seeder.SeedAdminUsersAsync();
        Environment.Exit(0);
    }
}
