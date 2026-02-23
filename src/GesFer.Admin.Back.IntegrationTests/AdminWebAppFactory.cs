using System.Collections.Generic;
using GesFer.Admin.Back.Api;
using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Infrastructure.Services;
using GesFer.Admin.Back.Domain.Services;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.Net.Http.Json;
using Xunit;

namespace GesFer.Admin.Back.IntegrationTests;

public class AdminWebAppFactory : WebApplicationFactory<Program>, IAsyncLifetime
{
    private readonly string _inMemoryDbName = "GesFerAdminTestDb_InMemory_" + Guid.NewGuid();

    protected override void ConfigureWebHost(IWebHostBuilder builder)
    {
        builder.ConfigureServices(services =>
        {
            // Remove existing DbContextOptions for AdminDbContext
            var dbContextOptionsDescriptors = services.Where(d => d.ServiceType == typeof(DbContextOptions<AdminDbContext>)).ToList();
            foreach (var descriptor in dbContextOptionsDescriptors) services.Remove(descriptor);

            var dbContextDescriptors = services.Where(d => d.ServiceType == typeof(AdminDbContext)).ToList();
            foreach (var descriptor in dbContextDescriptors) services.Remove(descriptor);

            // InMemory: ExecuteDeleteAsync no está soportado (PurgeLogs devuelve 500 en este entorno).
            // Para test relacional de PurgeLogs usar Testcontainers o SQLite con conexión compartida.
            services.AddDbContext<AdminDbContext>((serviceProvider, options) =>
            {
                options.UseInMemoryDatabase(_inMemoryDbName);
                options.EnableSensitiveDataLogging();
            }, ServiceLifetime.Scoped);

            // Register Seeder dependencies
            services.AddScoped<AdminJsonDataSeeder>();
            services.AddSingleton<ISensitiveDataSanitizer, SensitiveDataSanitizer>();
        });

        builder.UseEnvironment("Testing");

        // SharedSecret para tests (AuthorizeSystemOrAdmin)
        builder.ConfigureAppConfiguration((_, config) =>
        {
            config.AddInMemoryCollection(new Dictionary<string, string?>
            {
                ["SharedSecret"] = "test-internal-secret"
            });
        });
    }

    public async Task InitializeAsync()
    {
        using var scope = Services.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();
        await context.Database.EnsureCreatedAsync();

        // Run Seeders (Companies first - Admin SSOT; then Admin Users)
        var seeder = scope.ServiceProvider.GetRequiredService<AdminJsonDataSeeder>();
        await seeder.SeedAllAsync();
    }

    public new async Task DisposeAsync()
    {
        await base.DisposeAsync();
        await Serilog.Log.CloseAndFlushAsync();
    }

    public async Task<string> GetAdminAccessTokenAsync()
    {
        var client = CreateClient();
        var response = await client.PostAsJsonAsync("/api/admin/auth/login", new
        {
            Usuario = "admin",
            Contraseña = "admin123"
        });
        response.EnsureSuccessStatusCode();
        var result = await response.Content.ReadFromJsonAsync<GesFer.Admin.Back.Api.Controllers.AdminLoginResponse>();
        return result!.Token;
    }

    public async Task<HttpClient> GetAdminClientAsync()
    {
        var token = await GetAdminAccessTokenAsync();
        var client = CreateClient();
        client.DefaultRequestHeaders.Authorization = new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", token);
        return client;
    }
}
