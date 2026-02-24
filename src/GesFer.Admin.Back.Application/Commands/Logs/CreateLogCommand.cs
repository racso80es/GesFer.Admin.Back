using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using GesFer.Admin.Back.Domain.Entities;
using MediatR;
using System.Text.Json;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record CreateLogCommand(CreateLogDto Dto) : IRequest<Unit>;

public class CreateLogHandler : IRequestHandler<CreateLogCommand, Unit>
{
    private readonly IApplicationDbContext _context;

    public CreateLogHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<Unit> Handle(CreateLogCommand request, CancellationToken cancellationToken)
    {
        var entity = new Log
        {
            Level = request.Dto.Level,
            Message = request.Dto.Message,
            Exception = request.Dto.Exception,
            TimeStamp = request.Dto.TimeStamp,
            Properties = request.Dto.Properties != null
                ? JsonSerializer.Serialize(request.Dto.Properties)
                : null
        };

        _context.Logs.Add(entity);
        await _context.SaveChangesAsync(cancellationToken);

        return Unit.Value;
    }
}
