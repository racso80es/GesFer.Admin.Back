using GesFer.Admin.Back.Domain.Common;
using GesFer.Admin.Back.Domain.ValueObjects;

namespace GesFer.Admin.Back.Domain.Entities;

/// <summary>
/// Entidad de usuario (multi-tenant) asociada a una Company.
/// </summary>
public class User : BaseEntity
{
    public Guid CompanyId { get; set; }

    public string Username { get; set; } = string.Empty;
    public string PasswordHash { get; set; } = string.Empty;

    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;

    public Email? Email { get; set; }
    public string? Phone { get; set; }
    public string? Address { get; set; }

    public Guid? PostalCodeId { get; set; }
    public Guid? CityId { get; set; }
    public Guid? StateId { get; set; }
    public Guid? CountryId { get; set; }
    public Guid? LanguageId { get; set; }

    public PostalCode? PostalCode { get; set; }
    public City? City { get; set; }
    public State? State { get; set; }
    public Country? Country { get; set; }
    public Language? Language { get; set; }
}

