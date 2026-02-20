using GesFer.Admin.Back.Domain.Common;

namespace GesFer.Admin.Back.Domain.Entities;

/// <summary>
/// Entidad que representa una ciudad
/// </summary>
public class City : BaseEntity
{
    public Guid StateId { get; set; }
    public string Name { get; set; } = string.Empty;
    public State State { get; set; } = null!;
    public ICollection<PostalCode> PostalCodes { get; set; } = new List<PostalCode>();
}
