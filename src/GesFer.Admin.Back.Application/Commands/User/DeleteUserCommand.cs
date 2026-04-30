using MediatR;

namespace GesFer.Admin.Back.Application.Commands.User;

public record DeleteUserCommand(Guid Id) : IRequest;

