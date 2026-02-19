using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;

namespace GesFer.Admin.Back.Application.Commands.User;

public record CreateUserCommand(CreateUserDto Dto) : ICommand<UserDto>;
