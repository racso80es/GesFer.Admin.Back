using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Product.Application.DTOs.TaxTypes;

namespace GesFer.Admin.Back.Application.Commands.TaxTypes;

public record GetTaxTypeByIdCommand(Guid Id, Guid? CompanyId = null) : ICommand<TaxTypeDto?>;
