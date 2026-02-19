using GesFer.Admin.Domain.Common;

namespace GesFer.Admin.Domain.Entities;

/// <summary>
/// Entidad que representa un código postal
/// </summary>
public class PostalCode : BaseEntity
{
    public Guid CityId { get; set; }
    public string Code { get; set; } = string.Empty;
    public City City { get; set; } = null!;
}
