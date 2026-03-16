using GesFer.Admin.Back.Application.DTOs.Logs;
using Serilog.Core;
using Serilog.Events;

namespace GesFer.Admin.Back.Infrastructure.Logging;

public class MediatRLogSink : ILogEventSink
{
    private readonly ILogQueue _logQueue;
    private readonly IFormatProvider? _formatProvider;

    public MediatRLogSink(ILogQueue logQueue, IFormatProvider? formatProvider = null)
    {
        _logQueue = logQueue;
        _formatProvider = formatProvider;
    }

    public void Emit(LogEvent logEvent)
    {
        var dto = new CreateLogDto
        {
            Level = logEvent.Level.ToString(),
            Message = logEvent.RenderMessage(_formatProvider),
            Exception = logEvent.Exception?.ToString(),
            TimeStamp = logEvent.Timestamp.UtcDateTime,
            Properties = logEvent.Properties.ToDictionary(
                k => k.Key,
                v => (object)v.Value.ToString() // Simplify object extraction for EF Core
            )
        };

        // Try to enqueue synchronously, if it's full (or other reason) it might drop based on BoundedChannelFullMode
        // Since we are in a synchronous Emit method, we cannot await WriteAsync, we use TryWrite.
        _logQueue.TryWrite(dto);
    }
}
