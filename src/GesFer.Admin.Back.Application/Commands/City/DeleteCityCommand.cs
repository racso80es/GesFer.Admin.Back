using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.City;

public record DeleteCityCommand(Guid Id) : ICommand;
