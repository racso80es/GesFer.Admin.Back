using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.Auth;

/// <summary>
/// Comando para obtener los permisos de un usuario
/// </summary>
public class GetUserPermissionsCommand : ICommand<List<string>>
{
    public Guid UserId { get; set; }
}

