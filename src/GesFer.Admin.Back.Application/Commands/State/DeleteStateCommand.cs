using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.State;

public record DeleteStateCommand(Guid Id) : ICommand;
