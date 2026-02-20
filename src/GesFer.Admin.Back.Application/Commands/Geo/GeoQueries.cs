using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Geo;

public record GetAllCountriesCommand() : IRequest<List<CountryDto>>;

public record GetCountryByIdCommand(Guid Id) : IRequest<CountryDto?>;

public record GetStatesByCountryIdCommand(Guid CountryId) : IRequest<List<StateDto>>;

public record GetCitiesByStateIdCommand(Guid StateId) : IRequest<List<CityDto>>;
