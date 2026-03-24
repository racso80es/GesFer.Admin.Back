using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Geo;

public record GetAllCountriesCommand() : IRequest<IEnumerable<CountryGeoReadDto>>;

public record GetCountryByIdCommand(Guid Id) : IRequest<CountryGeoReadDto?>;

public record GetStatesByCountryIdCommand(Guid CountryId) : IRequest<IEnumerable<StateGeoReadDto>>;

public record GetCitiesByStateIdCommand(Guid StateId) : IRequest<IEnumerable<CityGeoReadDto>>;

public record GetPostalCodesByCityIdCommand(Guid CityId) : IRequest<IEnumerable<PostalCodeGeoReadDto>>;
