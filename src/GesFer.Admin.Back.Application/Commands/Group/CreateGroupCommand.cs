using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Group;

namespace GesFer.Admin.Back.Application.Commands.Group;

public record CreateGroupCommand(CreateGroupDto Dto) : ICommand<GroupDto>;
