using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;

namespace GesFer.Admin.Back.Application.Commands.User;

public record UpdateUserCommand(Guid Id, UpdateUserDto Dto) : ICommand<UserDto>;
