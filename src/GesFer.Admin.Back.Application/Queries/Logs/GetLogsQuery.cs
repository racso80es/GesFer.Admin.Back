using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Queries.Logs;

public record GetLogsQuery(
    DateTime? FromDate,
    DateTime? ToDate,
    string? Level,
    Guid? CompanyId,
    Guid? UserId,
    int PageNumber = 1,
    int PageSize = 10) : IRequest<LogsPagedResponseDto>;

public class GetLogsHandler : IRequestHandler<GetLogsQuery, LogsPagedResponseDto>
{
    private readonly IApplicationDbContext _context;

    public GetLogsHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<LogsPagedResponseDto> Handle(GetLogsQuery request, CancellationToken cancellationToken)
    {
        var query = _context.Logs.AsQueryable();

        if (request.FromDate.HasValue)
            query = query.Where(x => x.TimeStamp >= request.FromDate.Value);

        if (request.ToDate.HasValue)
            query = query.Where(x => x.TimeStamp <= request.ToDate.Value);

        if (!string.IsNullOrEmpty(request.Level))
            query = query.Where(x => x.Level == request.Level);

        if (request.CompanyId.HasValue)
            query = query.Where(x => x.CompanyId == request.CompanyId.Value);

        if (request.UserId.HasValue)
            query = query.Where(x => x.UserId == request.UserId.Value);

        var totalCount = await query.CountAsync(cancellationToken);

        var logs = await query
            .OrderByDescending(x => x.TimeStamp)
            .Skip((request.PageNumber - 1) * request.PageSize)
            .Take(request.PageSize)
            .ToListAsync(cancellationToken);

        var logDtos = logs.Select(x => new LogDto
        {
            Id = x.Id,
            Level = x.Level,
            Message = x.Message,
            Exception = x.Exception,
            TimeStamp = x.TimeStamp,
            Source = x.Source,
            CompanyId = x.CompanyId,
            UserId = x.UserId
        }).ToList();

        return new LogsPagedResponseDto
        {
            Logs = logDtos,
            TotalCount = totalCount,
            PageNumber = request.PageNumber,
            PageSize = request.PageSize,
            TotalPages = (int)Math.Ceiling(totalCount / (double)request.PageSize)
        };
    }
}
