using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Supplier;

namespace GesFer.Admin.Back.Application.Commands.Supplier;

public record GetAllSuppliersCommand(Guid? CompanyId = null) : ICommand<List<SupplierDto>>;
