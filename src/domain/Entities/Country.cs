using GesFer.Admin.Domain.Common;

namespace GesFer.Admin.Domain.Entities;

/// <summary>
/// Entidad que representa un país
/// </summary>
public class Country : BaseEntity
{
    public string Name { get; set; } = string.Empty;
    public string Code { get; set; } = string.Empty;
    public Guid LanguageId { get; set; }
    public Language? Language { get; set; }
    public ICollection<State> States { get; set; } = new List<State>();
}
