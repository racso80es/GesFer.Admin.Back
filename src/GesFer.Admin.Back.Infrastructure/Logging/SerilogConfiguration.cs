using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Hosting;
using Serilog;
using Serilog.Events;

namespace GesFer.Admin.Back.Infrastructure.Logging;

public static class SerilogConfiguration
{
    public static IHostBuilder ConfigureInfrastructureLogging(this IHostBuilder builder)
    {
        return builder.UseSerilog((context, services, configuration) =>
        {
            var isDevelopment = context.HostingEnvironment.IsDevelopment();
            var connectionString = context.Configuration.GetConnectionString("DefaultConnection");

            configuration
                .ReadFrom.Services(services)
                .Enrich.FromLogContext()
                .Enrich.WithProperty("Application", "GesFer.Admin.Back.Api")
                .Enrich.WithProperty("Environment", context.HostingEnvironment.EnvironmentName);

            if (isDevelopment)
            {
                configuration
                    .MinimumLevel.Verbose()
                    .WriteTo.Console();

                var useMySqlLogging = context.Configuration.GetValue<bool>("Serilog:UseMySql");
                if (useMySqlLogging && !string.IsNullOrEmpty(connectionString))
                {
                    configuration.WriteTo.MySQL(
                        connectionString: connectionString,
                        tableName: "Logs",
                        restrictedToMinimumLevel: LogEventLevel.Verbose,
                        storeTimestampInUtc: true);
                }
            }
            else
            {
                configuration
                    .MinimumLevel.Information()
                    .MinimumLevel.Override("Microsoft", LogEventLevel.Warning)
                    .MinimumLevel.Override("Microsoft.AspNetCore", LogEventLevel.Warning);

                if (!string.IsNullOrEmpty(connectionString))
                {
                    configuration.WriteTo.MySQL(
                        connectionString: connectionString,
                        tableName: "Logs",
                        restrictedToMinimumLevel: LogEventLevel.Information,
                        storeTimestampInUtc: true);
                }
            }
        });
    }
}
