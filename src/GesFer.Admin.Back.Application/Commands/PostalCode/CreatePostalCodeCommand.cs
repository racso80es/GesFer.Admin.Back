using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.PostalCode;

namespace GesFer.Admin.Back.Application.Commands.PostalCode;

public record CreatePostalCodeCommand(CreatePostalCodeDto Dto) : ICommand<PostalCodeDto>;
