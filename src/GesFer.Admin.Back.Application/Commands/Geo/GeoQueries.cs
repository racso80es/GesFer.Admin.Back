using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Geo;

public record GetAllCountriesCommand() : IRequest<List<CountryGeoReadDto>>;

public record GetCountryByIdCommand(Guid Id) : IRequest<CountryGeoReadDto?>;

public record GetStatesByCountryIdCommand(Guid CountryId) : IRequest<List<StateGeoReadDto>>;

public record GetCitiesByStateIdCommand(Guid StateId) : IRequest<List<CityGeoReadDto>>;

public record GetPostalCodesByCityIdCommand(Guid CityId) : IRequest<List<PostalCodeGeoReadDto>>;
