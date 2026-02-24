using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record PurgeLogsCommand(DateTime DateLimit) : IRequest<PurgeLogsResponseDto>;

public class PurgeLogsHandler : IRequestHandler<PurgeLogsCommand, PurgeLogsResponseDto>
{
    private readonly IApplicationDbContext _context;

    public PurgeLogsHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<PurgeLogsResponseDto> Handle(PurgeLogsCommand request, CancellationToken cancellationToken)
    {
        var sevenDaysAgo = DateTime.UtcNow.AddDays(-7);
        if (request.DateLimit > sevenDaysAgo)
        {
            throw new ArgumentException("No se pueden eliminar logs de los últimos 7 días");
        }

        var deletedCount = await _context.Logs
            .Where(x => x.TimeStamp < request.DateLimit)
            .ExecuteDeleteAsync(cancellationToken);

        return new PurgeLogsResponseDto
        {
            DeletedCount = deletedCount,
            DateLimit = request.DateLimit
        };
    }
}
