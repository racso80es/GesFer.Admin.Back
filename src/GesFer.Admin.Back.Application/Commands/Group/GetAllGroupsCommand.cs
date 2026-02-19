using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Group;

namespace GesFer.Admin.Back.Application.Commands.Group;

public record GetAllGroupsCommand() : ICommand<List<GroupDto>>;
