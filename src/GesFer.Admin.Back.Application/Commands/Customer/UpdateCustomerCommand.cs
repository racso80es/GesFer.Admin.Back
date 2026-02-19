using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Customer;

namespace GesFer.Admin.Back.Application.Commands.Customer;

public record UpdateCustomerCommand(Guid Id, UpdateCustomerDto Dto) : ICommand<CustomerDto>;
