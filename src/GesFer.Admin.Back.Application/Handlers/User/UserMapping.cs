using GesFer.Admin.Back.Application.DTOs.User;
using GesFer.Admin.Back.Domain.Entities;

namespace GesFer.Admin.Back.Application.Handlers.User;

internal static class UserMapping
{
    public static UserDto ToDto(this Domain.Entities.User user)
    {
        return new UserDto
        {
            Id = user.Id,
            CompanyId = user.CompanyId,
            Username = user.Username,
            FirstName = user.FirstName,
            LastName = user.LastName,
            Email = user.Email.HasValue ? user.Email.Value.Value : null,
            Phone = user.Phone,
            Address = user.Address,
            PostalCodeId = user.PostalCodeId,
            CityId = user.CityId,
            StateId = user.StateId,
            CountryId = user.CountryId,
            LanguageId = user.LanguageId,
            IsActive = user.IsActive,
            CreatedAt = user.CreatedAt,
            UpdatedAt = user.UpdatedAt
        };
    }
}

