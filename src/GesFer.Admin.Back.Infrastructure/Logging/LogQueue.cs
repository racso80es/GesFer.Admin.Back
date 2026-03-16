using GesFer.Admin.Back.Application.DTOs.Logs;
using System.Threading.Channels;

namespace GesFer.Admin.Back.Infrastructure.Logging;

public interface ILogQueue
{
    ValueTask WriteAsync(CreateLogDto logDto, CancellationToken cancellationToken = default);
    bool TryWrite(CreateLogDto logDto);
    IAsyncEnumerable<CreateLogDto> ReadAllAsync(CancellationToken cancellationToken = default);
}

public class LogQueue : ILogQueue
{
    private readonly Channel<CreateLogDto> _channel;

    public LogQueue()
    {
        var options = new BoundedChannelOptions(10000)
        {
            FullMode = BoundedChannelFullMode.DropOldest
        };
        _channel = Channel.CreateBounded<CreateLogDto>(options);
    }

    public async ValueTask WriteAsync(CreateLogDto logDto, CancellationToken cancellationToken = default)
    {
        await _channel.Writer.WriteAsync(logDto, cancellationToken);
    }

    public bool TryWrite(CreateLogDto logDto)
    {
        return _channel.Writer.TryWrite(logDto);
    }

    public IAsyncEnumerable<CreateLogDto> ReadAllAsync(CancellationToken cancellationToken = default)
    {
        return _channel.Reader.ReadAllAsync(cancellationToken);
    }
}
