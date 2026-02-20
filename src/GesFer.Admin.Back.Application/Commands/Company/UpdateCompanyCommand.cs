using GesFer.Admin.Back.Application.DTOs.Company;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Company;

public record UpdateCompanyCommand(Guid Id, UpdateCompanyDto Dto) : IRequest<CompanyDto>;
