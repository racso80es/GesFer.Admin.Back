using GesFer.Admin.Back.Api.Attributes;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;
using Microsoft.AspNetCore.Mvc;

namespace GesFer.Admin.Back.Api.Controllers;

/// <summary>
/// Controlador para gestión de estados/provincias
/// </summary>
[ApiController]
[Route("api/[controller]")]
[AuthorizeSystemOrAdmin]
public class StatesController : ControllerBase
{
    private readonly IMediator _mediator;
    private readonly ILogger<StatesController> _logger;

    public StatesController(IMediator mediator, ILogger<StatesController> logger)
    {
        _mediator = mediator;
        _logger = logger;
    }

    /// <summary>
    /// Obtiene las ciudades de un estado/provincia
    /// </summary>
    [HttpGet("{id}/cities")]
    [ProducesResponseType(typeof(List<CityDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetCities(Guid id)
    {
        try
        {
            var command = new GetCitiesByStateIdCommand(id);
            var result = await _mediator.Send(command);
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener ciudades del estado {Id}", id);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }
}
