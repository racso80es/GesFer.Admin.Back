using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;

namespace GesFer.Admin.Back.Application.Commands.User;

public record GetUserByIdCommand(Guid Id) : ICommand<UserDto?>;
