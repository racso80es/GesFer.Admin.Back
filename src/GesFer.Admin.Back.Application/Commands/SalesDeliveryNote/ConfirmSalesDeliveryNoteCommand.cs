using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.SalesDeliveryNote;

/// <summary>
/// Comando para confirmar un albarán de venta
/// </summary>
public class ConfirmSalesDeliveryNoteCommand : ICommand
{
    public Guid DeliveryNoteId { get; set; }
}

