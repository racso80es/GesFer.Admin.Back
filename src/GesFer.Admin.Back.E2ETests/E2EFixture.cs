using System.Collections.Generic;
using GesFer.Admin.Back.Infrastructure.Data;
using Xunit;
using GesFer.Admin.Back.Infrastructure.Services;
using GesFer.Admin.Back.Domain.Services;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

namespace GesFer.Admin.Back.E2ETests;

/// <summary>
/// Fixture E2E: API Admin con InMemory DB y seeds para tests de login y flujos completos.
/// </summary>
public class E2EFixture : WebApplicationFactory<Program>, IAsyncLifetime
{
    private readonly string _inMemoryDbName = "E2E_AdminDb_" + Guid.NewGuid();

    protected override void ConfigureWebHost(IWebHostBuilder builder)
    {
        builder.ConfigureServices(services =>
        {
            var dbContextOptionsDescriptors = services.Where(d => d.ServiceType == typeof(DbContextOptions<AdminDbContext>)).ToList();
            foreach (var d in dbContextOptionsDescriptors) services.Remove(d);

            var dbContextDescriptors = services.Where(d => d.ServiceType == typeof(AdminDbContext)).ToList();
            foreach (var d in dbContextDescriptors) services.Remove(d);

            services.AddDbContext<AdminDbContext>((_, options) =>
            {
                options.UseInMemoryDatabase(_inMemoryDbName);
                options.EnableSensitiveDataLogging();
            }, ServiceLifetime.Scoped);

            services.AddScoped<AdminJsonDataSeeder>();
            services.AddSingleton<ISensitiveDataSanitizer, SensitiveDataSanitizer>();
        });

        builder.UseEnvironment("Testing");

        builder.ConfigureAppConfiguration((_, config) =>
        {
            config.AddInMemoryCollection(new Dictionary<string, string?>
            {
                ["SharedSecret"] = "e2e-internal-secret"
            });
        });
    }

    public async Task InitializeAsync()
    {
        using var scope = Services.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();
        await context.Database.EnsureCreatedAsync();

        var seeder = scope.ServiceProvider.GetRequiredService<AdminJsonDataSeeder>();
        await seeder.SeedAllAsync();
    }

    public new async Task DisposeAsync()
    {
        await base.DisposeAsync();
        await Serilog.Log.CloseAndFlushAsync();
    }
}
