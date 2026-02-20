using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Company;

public record DeleteCompanyCommand(Guid Id) : IRequest;
