using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Domain.Entities;

namespace GesFer.Admin.Back.Application.Commands.PurchaseDeliveryNote;

/// <summary>
/// Comando para crear un albarán de compra
/// </summary>
public class CreatePurchaseDeliveryNoteCommand : ICommand<GesFer.Admin.Back.Domain.Entities.PurchaseDeliveryNote>
{
    public Guid CompanyId { get; set; }
    public Guid SupplierId { get; set; }
    public DateTime Date { get; set; }
    public string? Reference { get; set; }
    public List<PurchaseDeliveryNoteLineDto> Lines { get; set; } = new();
}

/// <summary>
/// DTO para crear líneas de albarán de compra
/// </summary>
public class PurchaseDeliveryNoteLineDto
{
    public Guid ArticleId { get; set; }
    public decimal Quantity { get; set; }
    public decimal? Price { get; set; } // Si es null, se toma de la tarifa del proveedor o del artículo base
}

