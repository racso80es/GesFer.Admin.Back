using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.PurchaseDeliveryNote;

/// <summary>
/// Comando para confirmar un albarán de compra
/// </summary>
public class ConfirmPurchaseDeliveryNoteCommand : ICommand
{
    public Guid DeliveryNoteId { get; set; }
}

