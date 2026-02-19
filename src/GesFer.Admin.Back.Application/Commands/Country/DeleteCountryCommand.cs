using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.Country;

public record DeleteCountryCommand(Guid Id) : ICommand;
