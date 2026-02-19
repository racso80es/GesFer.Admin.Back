using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.Supplier;

public record DeleteSupplierCommand(Guid Id) : ICommand;
