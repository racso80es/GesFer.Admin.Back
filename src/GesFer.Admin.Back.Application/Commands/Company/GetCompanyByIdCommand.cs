using GesFer.Admin.Back.Application.DTOs.Company;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Company;

public record GetCompanyByIdCommand(Guid Id) : IRequest<CompanyDto?>;
