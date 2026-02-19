using GesFer.Admin.Domain.Common;
using GesFer.Admin.Domain.ValueObjects;

namespace GesFer.Admin.Domain.Entities;

/// <summary>
/// Entidad que representa una empresa (Tenant) en el sistema.
/// </summary>
public class Company : BaseEntity
{
    public string Name { get; set; } = string.Empty;
    public TaxId? TaxId { get; set; }
    public string Address { get; set; } = string.Empty;
    public string? Phone { get; set; }
    public Email? Email { get; set; }
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
