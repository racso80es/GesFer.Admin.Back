using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Infrastructure.Logging;

public sealed class LogQueueLoggerProvider : ILoggerProvider
{
    private readonly ILogQueue _logQueue;

    public LogQueueLoggerProvider(ILogQueue logQueue)
    {
        _logQueue = logQueue;
    }

    public ILogger CreateLogger(string categoryName) => new LogQueueLogger(categoryName, _logQueue);

    public void Dispose() { }
}
