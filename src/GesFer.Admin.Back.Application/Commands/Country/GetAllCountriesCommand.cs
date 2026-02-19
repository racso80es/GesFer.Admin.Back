using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Country;

namespace GesFer.Admin.Back.Application.Commands.Country;

public record GetAllCountriesCommand() : ICommand<List<CountryDto>>;
