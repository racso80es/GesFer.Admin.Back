using GesFer.Admin.Back.Application.DTOs.User;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.User;

public record GetUserByIdCommand(Guid Id, Guid CompanyId) : IRequest<UserDto?>;

