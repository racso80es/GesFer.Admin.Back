using GesFer.Admin.Back.Application.Commands.ArticleFamilies;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.ArticleFamilies;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.ArticleFamilies;

public class GetArticleFamilyByIdCommandHandler : ICommandHandler<GetArticleFamilyByIdCommand, ArticleFamilyDto?>
{
    private readonly ApplicationDbContext _context;

    public GetArticleFamilyByIdCommandHandler(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<ArticleFamilyDto?> HandleAsync(GetArticleFamilyByIdCommand command, CancellationToken cancellationToken = default)
    {
        var query = _context.ArticleFamilies
            .Include(af => af.TaxType)
            .Where(af => af.Id == command.Id && af.DeletedAt == null);

        if (command.CompanyId.HasValue)
            query = query.Where(af => af.CompanyId == command.CompanyId.Value);

        var entity = await query
            .Select(af => new ArticleFamilyDto
            {
                Id = af.Id,
                CompanyId = af.CompanyId,
                Code = af.Code,
                Name = af.Name,
                Description = af.Description,
                TaxTypeId = af.TaxTypeId,
                TaxTypeName = af.TaxType != null ? af.TaxType.Name : null,
                TaxTypeValue = af.TaxType != null ? af.TaxType.Value : null,
                IsActive = af.IsActive,
                CreatedAt = af.CreatedAt,
                UpdatedAt = af.UpdatedAt
            })
            .FirstOrDefaultAsync(cancellationToken);

        return entity;
    }
}
