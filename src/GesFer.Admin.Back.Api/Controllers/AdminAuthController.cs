using GesFer.Admin.Back.Application.Commands.Auth;
using GesFer.Admin.Back.Application.DTOs.Auth;
using MediatR;
using Microsoft.AspNetCore.Mvc;

namespace GesFer.Admin.Back.Api.Controllers;

/// <summary>
/// Controlador para autenticación administrativa. Delega al AdminLoginHandler (CQRS).
/// </summary>
[ApiController]
[Route("api/admin/auth")]
public class AdminAuthController : ControllerBase
{
    private readonly ISender _sender;
    private readonly ILogger<AdminAuthController> _logger;

    public AdminAuthController(ISender sender, ILogger<AdminAuthController> logger)
    {
        _sender = sender;
        _logger = logger;
    }

    /// <summary>
    /// Realiza el login del usuario administrativo
    /// </summary>
    /// <param name="request">Datos de login administrativo (Usuario, Contraseña)</param>
    /// <returns>Información del usuario administrativo autenticado y token JWT con role: Admin</returns>
    [HttpPost("login")]
    [ProducesResponseType(typeof(AdminLoginResponse), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    [ProducesResponseType(StatusCodes.Status401Unauthorized)]
    [ProducesResponseType(StatusCodes.Status500InternalServerError)]
    public async Task<IActionResult> Login([FromBody] AdminLoginRequest request)
    {
        try
        {
            var clientIp = HttpContext.Connection.RemoteIpAddress?.ToString();
            var userAgent = HttpContext.Request.Headers["User-Agent"].FirstOrDefault();

            var command = new AdminLoginCommand(
                request.Usuario ?? string.Empty,
                request.Contraseña ?? string.Empty,
                clientIp,
                userAgent);

            var result = await _sender.Send(command);

            if (result.IsSuccess && result.Response != null)
                return Ok(result.Response);

            return result.HttpStatusCode switch
            {
                400 => BadRequest(new { message = result.ErrorMessage }),
                401 => Unauthorized(new { message = result.ErrorMessage }),
                _ => StatusCode(result.HttpStatusCode, new { message = result.ErrorMessage })
            };
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al realizar login administrativo para usuario: {Usuario}", request?.Usuario);
            return StatusCode(500, new { message = "Error interno del servidor", error = ex.Message });
        }
    }
}
