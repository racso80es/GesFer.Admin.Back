using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Queries.Logs;

public record GetAuditLogsQuery(
    string? Action,
    string? Username,
    int PageNumber = 1,
    int PageSize = 50) : IRequest<AuditLogsPagedResponseDto>;

public class GetAuditLogsHandler : IRequestHandler<GetAuditLogsQuery, AuditLogsPagedResponseDto>
{
    private readonly IApplicationDbContext _context;

    public GetAuditLogsHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<AuditLogsPagedResponseDto> Handle(GetAuditLogsQuery request, CancellationToken cancellationToken)
    {
        var query = _context.AuditLogs.AsQueryable();

        if (!string.IsNullOrEmpty(request.Action))
            query = query.Where(x => x.Action == request.Action);

        if (!string.IsNullOrEmpty(request.Username))
            query = query.Where(x => x.Username == request.Username);

        var totalCount = await query.CountAsync(cancellationToken);

        var logs = await query
            .OrderByDescending(x => x.ActionTimestamp)
            .Skip((request.PageNumber - 1) * request.PageSize)
            .Take(request.PageSize)
            .ToListAsync(cancellationToken);

        var auditLogDtos = logs.Select(x => new AuditLogDto
        {
            Id = x.Id,
            CursorId = x.CursorId,
            Username = x.Username,
            Action = x.Action,
            HttpMethod = x.HttpMethod,
            Path = x.Path,
            AdditionalData = x.AdditionalData,
            ActionTimestamp = x.ActionTimestamp
        }).ToList();

        return new AuditLogsPagedResponseDto
        {
            AuditLogs = auditLogDtos,
            TotalCount = totalCount,
            PageNumber = request.PageNumber,
            PageSize = request.PageSize,
            TotalPages = (int)Math.Ceiling(totalCount / (double)request.PageSize)
        };
    }
}
