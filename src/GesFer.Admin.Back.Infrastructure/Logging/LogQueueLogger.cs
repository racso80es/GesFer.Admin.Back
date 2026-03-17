using System.Collections.Generic;
using GesFer.Admin.Back.Application.DTOs.Logs;
using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Infrastructure.Logging;

internal sealed class LogQueueLogger : ILogger
{
    private readonly string _categoryName;
    private readonly ILogQueue _logQueue;

    public LogQueueLogger(string categoryName, ILogQueue logQueue)
    {
        _categoryName = categoryName;
        _logQueue = logQueue;
    }

    public IDisposable? BeginScope<TState>(TState state) where TState : notnull => default;

    public bool IsEnabled(LogLevel logLevel) => logLevel != LogLevel.None;

    public void Log<TState>(
        LogLevel logLevel,
        EventId eventId,
        TState state,
        Exception? exception,
        Func<TState, Exception?, string> formatter)
    {
        if (!IsEnabled(logLevel))
            return;

        // Evitar loop infinito: si es EF Core, Microsoft, o el propio servicio de logs guardando en BD
        if (_categoryName.StartsWith("Microsoft") ||
            _categoryName.StartsWith("System") ||
            _categoryName.Contains("LogDispatcherBackgroundService") ||
            _categoryName.Contains("CreateLogHandler"))
        {
            return;
        }

        var message = formatter(state, exception);
        var properties = ExtractProperties(state, eventId);

        var dto = new CreateLogDto
        {
            Level = logLevel.ToString(),
            Message = message,
            Exception = exception?.ToString(),
            TimeStamp = DateTime.UtcNow,
            Properties = properties.Count > 0 ? properties : null
        };

        _logQueue.TryWrite(dto);
    }

    private static Dictionary<string, object> ExtractProperties<TState>(TState state, EventId eventId)
    {
        var dict = new Dictionary<string, object>();

        if (eventId.Id != 0 || !string.IsNullOrEmpty(eventId.Name))
        {
            dict["EventId"] = eventId.Id;
            if (!string.IsNullOrEmpty(eventId.Name))
                dict["EventName"] = eventId.Name;
        }

        if (state is IReadOnlyList<KeyValuePair<string, object?>> list)
        {
            foreach (var kv in list)
            {
                if (string.IsNullOrEmpty(kv.Key))
                    continue;
                if (kv.Key == "{OriginalFormat}")
                    continue;
                dict[kv.Key] = kv.Value ?? string.Empty;
            }
        }

        return dict;
    }
}
