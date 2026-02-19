using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.City;

namespace GesFer.Admin.Back.Application.Commands.City;

public record UpdateCityCommand(Guid Id, UpdateCityDto Dto) : ICommand<CityDto>;
