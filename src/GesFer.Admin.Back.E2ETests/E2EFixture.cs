using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Mvc.Testing;
using Microsoft.AspNetCore.TestHost;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using GesFer.Admin.Back.Infrastructure.Data;

namespace GesFer.Admin.Back.E2ETests;

public class E2EFixture : WebApplicationFactory<Program>
{
    protected override void ConfigureWebHost(IWebHostBuilder builder)
    {
        builder.ConfigureServices(services =>
        {
            // Remover DbContext real
            var descriptor = services.SingleOrDefault(d => d.ServiceType == typeof(DbContextOptions<AdminDbContext>));
            if (descriptor != null) services.Remove(descriptor);

            // Agregar InMemory DbContext
            services.AddDbContext<AdminDbContext>(options =>
            {
                options.UseInMemoryDatabase("E2E_AdminDb_" + Guid.NewGuid());
            });

            // Remover Serilog MySQL Sink si existe (para evitar errores de conexión)
             var serviceDescriptor = services.FirstOrDefault(descriptor => descriptor.ServiceType == typeof(ILoggerProvider));
             if (serviceDescriptor != null) services.Remove(serviceDescriptor);

        });

        builder.UseEnvironment("Testing");
    }
}
