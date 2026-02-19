using GesFer.Admin.Domain.Common;

namespace GesFer.Admin.Domain.Entities;

/// <summary>
/// Idioma maestro del sistema.
/// </summary>
public class Language : BaseEntity
{
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
    public string? Description { get; set; }
    public ICollection<Country> Countries { get; set; } = new List<Country>();
}
