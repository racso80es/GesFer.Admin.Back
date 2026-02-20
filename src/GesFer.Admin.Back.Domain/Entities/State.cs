using GesFer.Admin.Back.Domain.Common;

namespace GesFer.Admin.Back.Domain.Entities;

/// <summary>
/// Entidad que representa una provincia/estado
/// </summary>
public class State : BaseEntity
{
    public Guid CountryId { get; set; }
    public string Name { get; set; } = string.Empty;
    public string? Code { get; set; }
    public Country Country { get; set; } = null!;
    public ICollection<City> Cities { get; set; } = new List<City>();
}
