using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;

namespace GesFer.Admin.Back.E2ETests;

/// <summary>
/// Fixture E2E que arranca la API en memoria usando WebApplicationFactory.
/// Esto elimina la necesidad de tener un servidor corriendo externamente en CI.
/// Reemplaza la base de datos MySQL por InMemory para pruebas aisladas.
/// </summary>
public sealed class E2EFixture : WebApplicationFactory<Program>
{
    public E2EFixture()
    {
        // Limpiar logger estático de Serilog para evitar el error "The logger is already frozen"
        // cuando WebApplicationFactory arranca múltiples veces o en paralelo.
        Serilog.Log.CloseAndFlush();

        // Establecer la variable de entorno para que Program.cs detecte que estamos en modo Testing
        // y no configure Serilog (BootstrapLogger) de forma conflictiva.
        Environment.SetEnvironmentVariable("ASPNETCORE_ENVIRONMENT", "Testing");

        // Establecer SharedSecret via variable de entorno para que la API lo recoja.
        Environment.SetEnvironmentVariable("SharedSecret", E2EContext.InternalSecret);
    }

    protected override void Dispose(bool disposing)
    {
        if (disposing)
        {
            Environment.SetEnvironmentVariable("ASPNETCORE_ENVIRONMENT", null);
            Environment.SetEnvironmentVariable("SharedSecret", null);
        }
        base.Dispose(disposing);
    }

    protected override void ConfigureWebHost(IWebHostBuilder builder)
    {
        builder.ConfigureServices(services =>
        {
            // Eliminar DbContext configurado para MySQL
            var descriptor = services.SingleOrDefault(
                d => d.ServiceType == typeof(DbContextOptions<AdminDbContext>));

            if (descriptor != null)
            {
                services.Remove(descriptor);
            }

            // Añadir DbContext en memoria
            services.AddDbContext<AdminDbContext>(options =>
            {
                options.UseInMemoryDatabase("InMemoryDbForTesting");
                options.EnableSensitiveDataLogging();
            });

            // Reemplazar IApplicationDbContext para usar el nuevo contexto en memoria
            services.RemoveAll<IApplicationDbContext>();
            services.AddScoped<IApplicationDbContext>(provider =>
                provider.GetRequiredService<AdminDbContext>());

            // Asegurar que se creen seeds en la base de datos en memoria
            var sp = services.BuildServiceProvider();
            using var scope = sp.CreateScope();
            var scopedServices = scope.ServiceProvider;
            var db = scopedServices.GetRequiredService<AdminDbContext>();
            db.Database.EnsureCreated();

            // Aquí podríamos llamar al seeder manualmente si Program.cs no lo hace
            // Pero Program.cs llama a RunMigrationsAndSeedsAsync.
            // OJO: RunMigrationsAndSeedsAsync intenta ejecutar migraciones.
            // InMemory no soporta migraciones relacionales.
            // Necesitamos interceptar esa llamada o asegurar que maneje InMemory.
            // Program.cs comprueba: await app.Services.RunMigrationsAndSeedsAsync();
        });

        builder.UseEnvironment("Testing");
    }
}
