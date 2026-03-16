using GesFer.Admin.Back.Application.Commands.Logs;
using MediatR;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Infrastructure.Logging;

public class LogDispatcherBackgroundService : BackgroundService
{
    private readonly ILogQueue _logQueue;
    private readonly IServiceProvider _serviceProvider;
    private readonly ILogger<LogDispatcherBackgroundService> _logger;

    public LogDispatcherBackgroundService(
        ILogQueue logQueue,
        IServiceProvider serviceProvider,
        ILogger<LogDispatcherBackgroundService> logger)
    {
        _logQueue = logQueue;
        _serviceProvider = serviceProvider;
        _logger = logger;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        _logger.LogInformation("LogDispatcherBackgroundService is starting.");

        try
        {
            await foreach (var logDto in _logQueue.ReadAllAsync(stoppingToken))
            {
                try
                {
                    using var scope = _serviceProvider.CreateScope();
                    var sender = scope.ServiceProvider.GetRequiredService<ISender>();

                    var command = new CreateLogCommand(logDto);
                    await sender.Send(command, stoppingToken);
                }
                catch (Exception ex)
                {
                    // Evitar reenviar el error a la cola de logs para evitar bucles infinitos
                    Console.WriteLine($"[CRITICAL] Error in LogDispatcherBackgroundService processing log: {ex}");
                }
            }
        }
        catch (OperationCanceledException)
        {
            _logger.LogInformation("LogDispatcherBackgroundService is stopping due to cancellation.");
        }
        catch (Exception ex)
        {
            _logger.LogCritical(ex, "LogDispatcherBackgroundService encountered a fatal error.");
        }
    }
}
