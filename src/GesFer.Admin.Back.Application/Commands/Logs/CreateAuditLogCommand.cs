using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record CreateAuditLogCommand(CreateAuditLogDto Dto) : IRequest<Unit>;

public class CreateAuditLogHandler : IRequestHandler<CreateAuditLogCommand, Unit>
{
    private readonly IAuditLogService _auditService;

    public CreateAuditLogHandler(IAuditLogService auditService)
    {
        _auditService = auditService;
    }

    public async Task<Unit> Handle(CreateAuditLogCommand request, CancellationToken cancellationToken)
    {
        await _auditService.LogActionAsync(
            request.Dto.CursorId ?? string.Empty,
            request.Dto.Username ?? string.Empty,
            request.Dto.Action ?? string.Empty,
            request.Dto.HttpMethod ?? string.Empty,
            request.Dto.Path ?? string.Empty,
            request.Dto.AdditionalData
        );
        return Unit.Value;
    }
}
