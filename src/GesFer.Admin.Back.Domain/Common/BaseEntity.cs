namespace GesFer.Admin.Back.Domain.Common;

/// <summary>
/// Clase base para todas las entidades del dominio.
/// Implementa Soft Delete y auditoría básica.
/// </summary>
public abstract class BaseEntity
{
    public Guid Id { get; set; }
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
    public DateTime? UpdatedAt { get; set; }
    public DateTime? DeletedAt { get; set; }
    public bool IsActive { get; set; } = true;
    public bool IsDeleted => DeletedAt.HasValue;
}
