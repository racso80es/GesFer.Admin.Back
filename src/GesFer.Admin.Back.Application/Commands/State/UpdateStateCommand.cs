using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.State;

namespace GesFer.Admin.Back.Application.Commands.State;

public record UpdateStateCommand(Guid Id, UpdateStateDto Dto) : ICommand<StateDto>;
