using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.Customer;

public record DeleteCustomerCommand(Guid Id) : ICommand;
