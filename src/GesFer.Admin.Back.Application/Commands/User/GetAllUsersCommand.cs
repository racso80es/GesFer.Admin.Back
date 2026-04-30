using GesFer.Admin.Back.Application.DTOs.User;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.User;

public record GetAllUsersCommand(Guid CompanyId) : IRequest<List<UserDto>>;

