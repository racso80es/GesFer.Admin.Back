using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.TaxTypes;

public record DeleteTaxTypeCommand(Guid Id, Guid? CompanyId = null) : ICommand;
