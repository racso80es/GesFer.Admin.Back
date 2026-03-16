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

            // Resolve the queue created in Program.cs to provide it to the sink
            var logQueue = services.GetService(typeof(ILogQueue)) as ILogQueue;

            if (isDevelopment)
            {
                configuration
                    .MinimumLevel.Verbose()
                    .WriteTo.Console();

                if (logQueue != null)
                {
                    configuration.WriteTo.Sink(new MediatRLogSink(logQueue), LogEventLevel.Verbose);
                }
            }
            else
            {
                configuration
                    .MinimumLevel.Information()
                    .MinimumLevel.Override("Microsoft", LogEventLevel.Warning)
                    .MinimumLevel.Override("Microsoft.AspNetCore", LogEventLevel.Warning);

                if (logQueue != null)
                {
                    configuration.WriteTo.Sink(new MediatRLogSink(logQueue), LogEventLevel.Information);
                }
            }
        });
    }
}
