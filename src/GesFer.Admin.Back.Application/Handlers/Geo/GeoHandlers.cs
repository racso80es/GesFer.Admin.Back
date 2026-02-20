using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Geo;

public class GetAllCountriesHandler : IRequestHandler<GetAllCountriesCommand, List<CountryDto>>
{
    private readonly IApplicationDbContext _context;

    public GetAllCountriesHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<List<CountryDto>> Handle(GetAllCountriesCommand request, CancellationToken cancellationToken)
    {
        return await _context.Countries
            .AsNoTracking()
            .Where(c => c.DeletedAt == null)
            .OrderBy(c => c.Name)
            .Select(c => new CountryDto
            {
                Id = c.Id,
                Name = c.Name,
                Code = c.Code,
                LanguageId = c.LanguageId
            })
            .ToListAsync(cancellationToken);
    }
}

public class GetCountryByIdHandler : IRequestHandler<GetCountryByIdCommand, CountryDto?>
{
    private readonly IApplicationDbContext _context;

    public GetCountryByIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<CountryDto?> Handle(GetCountryByIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.Countries
            .AsNoTracking()
            .Where(c => c.Id == request.Id && c.DeletedAt == null)
            .Select(c => new CountryDto
            {
                Id = c.Id,
                Name = c.Name,
                Code = c.Code,
                LanguageId = c.LanguageId
            })
            .FirstOrDefaultAsync(cancellationToken);
    }
}

public class GetStatesByCountryIdHandler : IRequestHandler<GetStatesByCountryIdCommand, List<StateDto>>
{
    private readonly IApplicationDbContext _context;

    public GetStatesByCountryIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<List<StateDto>> Handle(GetStatesByCountryIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.States
            .AsNoTracking()
            .Where(s => s.CountryId == request.CountryId && s.DeletedAt == null)
            .OrderBy(s => s.Name)
            .Select(s => new StateDto
            {
                Id = s.Id,
                CountryId = s.CountryId,
                Name = s.Name,
                Code = s.Code
            })
            .ToListAsync(cancellationToken);
    }
}

public class GetCitiesByStateIdHandler : IRequestHandler<GetCitiesByStateIdCommand, List<CityDto>>
{
    private readonly IApplicationDbContext _context;

    public GetCitiesByStateIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<List<CityDto>> Handle(GetCitiesByStateIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.Cities
            .AsNoTracking()
            .Where(c => c.StateId == request.StateId && c.DeletedAt == null)
            .OrderBy(c => c.Name)
            .Select(c => new CityDto
            {
                Id = c.Id,
                StateId = c.StateId,
                Name = c.Name
            })
            .ToListAsync(cancellationToken);
    }
}
