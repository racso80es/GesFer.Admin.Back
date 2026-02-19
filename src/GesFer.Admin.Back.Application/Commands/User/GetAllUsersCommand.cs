using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;

namespace GesFer.Admin.Back.Application.Commands.User;

public record GetAllUsersCommand(Guid? CompanyId = null) : ICommand<List<UserDto>>;
