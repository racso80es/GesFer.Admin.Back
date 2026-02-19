using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.PostalCode;

namespace GesFer.Admin.Back.Application.Commands.PostalCode;

public record GetAllPostalCodesCommand(Guid? CityId = null, Guid? StateId = null, Guid? CountryId = null) : ICommand<List<PostalCodeDto>>;
