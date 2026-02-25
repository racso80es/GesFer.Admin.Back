using GesFer.Admin.Back.Api.Attributes;
using GesFer.Admin.Back.Application.Commands.Logs;
using GesFer.Admin.Back.Application.DTOs.Logs;
using GesFer.Admin.Back.Application.Queries.Logs;
using MediatR;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace GesFer.Admin.Back.Api.Controllers;

/// <summary>
/// Controlador para gestión de logs del sistema. Delega en MediatR (CQRS).
/// </summary>
[ApiController]
[Route("api/admin/logs")]
public class LogController : ControllerBase
{
    private readonly ISender _sender;
    private readonly ILogger<LogController> _logger;

    public LogController(ISender sender, ILogger<LogController> logger)
    {
        _sender = sender;
        _logger = logger;
    }

    /// <summary>
    /// Recibe un log desde otros servicios (System)
    /// </summary>
    [HttpPost]
    [AuthorizeSystemOrAdmin]
    [ProducesResponseType(StatusCodes.Status200OK)]
    public async Task<IActionResult> ReceiveLog([FromBody] CreateLogDto dto)
    {
        try
        {
            await _sender.Send(new CreateLogCommand(dto));
            return Ok();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al guardar log recibido");
            return StatusCode(500, new { message = "Error interno al guardar log" });
        }
    }

    /// <summary>
    /// Recibe un log de auditoría desde otros servicios (System)
    /// </summary>
    [HttpPost]
    [Route("/api/admin/audit-logs")]
    [AuthorizeSystemOrAdmin]
    [ProducesResponseType(StatusCodes.Status200OK)]
    public async Task<IActionResult> ReceiveAuditLog([FromBody] CreateAuditLogDto dto)
    {
        try
        {
            await _sender.Send(new CreateAuditLogCommand(dto));
            return Ok();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al guardar audit log recibido");
            return StatusCode(500, new { message = "Error interno al guardar audit log" });
        }
    }

    /// <summary>
    /// Obtiene logs paginados con filtros opcionales
    /// </summary>
    [HttpGet]
    [Authorize(Policy = "AdminOnly")]
    [ProducesResponseType(typeof(LogsPagedResponseDto), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetLogs(
        [FromQuery] DateTime? fromDate = null,
        [FromQuery] DateTime? toDate = null,
        [FromQuery] string? level = null,
        [FromQuery] Guid? companyId = null,
        [FromQuery] Guid? userId = null,
        [FromQuery] int pageNumber = 1,
        [FromQuery] int pageSize = 50)
    {
        try
        {
            var result = await _sender.Send(new Application.Queries.Logs.GetLogsQuery(fromDate, toDate, level, companyId, userId, pageNumber, pageSize));
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener logs");
            return StatusCode(500, new { message = "Error interno del servidor", error = ex.Message });
        }
    }

    /// <summary>
    /// Purga logs antiguos anteriores a la fecha límite especificada
    /// </summary>
    [HttpDelete]
    [Authorize(Policy = "AdminOnly")]
    [ProducesResponseType(typeof(PurgeLogsResponseDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<IActionResult> PurgeLogs([FromQuery] DateTime dateLimit)
    {
        try
        {
            var result = await _sender.Send(new PurgeLogsCommand(dateLimit));
            return Ok(result);
        }
        catch (ArgumentException ex)
        {
             return BadRequest(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al purgar logs con fecha límite: {DateLimit}", dateLimit);
            return StatusCode(500, new { message = "Error interno del servidor", error = ex.Message });
        }
    }
}
