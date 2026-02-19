using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.User;

public record DeleteUserCommand(Guid Id) : ICommand;
