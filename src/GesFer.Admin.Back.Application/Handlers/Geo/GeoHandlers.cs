using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Geo;

public sealed class GetAllCountriesHandler : IRequestHandler<GetAllCountriesCommand, IEnumerable<CountryGeoReadDto>>
{
    private readonly IApplicationDbContext _context;

    public GetAllCountriesHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<IEnumerable<CountryGeoReadDto>> Handle(GetAllCountriesCommand request, CancellationToken cancellationToken)
    {
        return await _context.Countries
            .AsNoTracking()
            .IgnoreQueryFilters()
            .Where(c => c.IsActive)
            .OrderBy(c => c.Name)
            .Select(c => new CountryGeoReadDto
            {
                Id = c.Id,
                Name = c.Name,
                Code = c.Code
            })
            .ToListAsync(cancellationToken);
    }
}

public sealed class GetCountryByIdHandler : IRequestHandler<GetCountryByIdCommand, CountryGeoReadDto?>
{
    private readonly IApplicationDbContext _context;

    public GetCountryByIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<CountryGeoReadDto?> Handle(GetCountryByIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.Countries
            .AsNoTracking()
            .IgnoreQueryFilters()
            .Where(c => c.Id == request.Id && c.IsActive)
            .Select(c => new CountryGeoReadDto
            {
                Id = c.Id,
                Name = c.Name,
                Code = c.Code
            })
            .FirstOrDefaultAsync(cancellationToken);
    }
}

public sealed class GetStatesByCountryIdHandler : IRequestHandler<GetStatesByCountryIdCommand, IEnumerable<StateGeoReadDto>>
{
    private readonly IApplicationDbContext _context;

    public GetStatesByCountryIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<IEnumerable<StateGeoReadDto>> Handle(GetStatesByCountryIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.States
            .AsNoTracking()
            .IgnoreQueryFilters()
            .Where(s => s.CountryId == request.CountryId && s.IsActive)
            .OrderBy(s => s.Name)
            .Select(s => new StateGeoReadDto
            {
                Id = s.Id,
                CountryId = s.CountryId,
                Name = s.Name,
                Code = s.Code
            })
            .ToListAsync(cancellationToken);
    }
}

public sealed class GetCitiesByStateIdHandler : IRequestHandler<GetCitiesByStateIdCommand, IEnumerable<CityGeoReadDto>>
{
    private readonly IApplicationDbContext _context;

    public GetCitiesByStateIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<IEnumerable<CityGeoReadDto>> Handle(GetCitiesByStateIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.Cities
            .AsNoTracking()
            .IgnoreQueryFilters()
            .Where(c => c.StateId == request.StateId && c.IsActive)
            .OrderBy(c => c.Name)
            .Select(c => new CityGeoReadDto
            {
                Id = c.Id,
                StateId = c.StateId,
                Name = c.Name
            })
            .ToListAsync(cancellationToken);
    }
}

public sealed class GetPostalCodesByCityIdHandler : IRequestHandler<GetPostalCodesByCityIdCommand, IEnumerable<PostalCodeGeoReadDto>>
{
    private readonly IApplicationDbContext _context;

    public GetPostalCodesByCityIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<IEnumerable<PostalCodeGeoReadDto>> Handle(GetPostalCodesByCityIdCommand request, CancellationToken cancellationToken)
    {
        return await _context.PostalCodes
            .AsNoTracking()
            .IgnoreQueryFilters()
            .Where(p => p.CityId == request.CityId && p.IsActive)
            .OrderBy(p => p.Code)
            .Select(p => new PostalCodeGeoReadDto
            {
                Id = p.Id,
                CityId = p.CityId,
                Code = p.Code
            })
            .ToListAsync(cancellationToken);
    }
}
