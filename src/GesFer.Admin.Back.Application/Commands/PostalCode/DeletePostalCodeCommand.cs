using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.PostalCode;

public record DeletePostalCodeCommand(Guid Id) : ICommand;
