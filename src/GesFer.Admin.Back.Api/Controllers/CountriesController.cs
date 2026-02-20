using GesFer.Admin.Back.Api.Attributes;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;
using Microsoft.AspNetCore.Mvc;

namespace GesFer.Admin.Back.Api.Controllers;

/// <summary>
/// Controlador para gestión de países
/// </summary>
[ApiController]
[Route("api/[controller]")]
[AuthorizeSystemOrAdmin]
public class CountriesController : ControllerBase
{
    private readonly IMediator _mediator;
    private readonly ILogger<CountriesController> _logger;

    public CountriesController(IMediator mediator, ILogger<CountriesController> logger)
    {
        _mediator = mediator;
        _logger = logger;
    }

    /// <summary>
    /// Obtiene todos los países
    /// </summary>
    [HttpGet]
    [ProducesResponseType(typeof(List<CountryDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetAll()
    {
        try
        {
            var command = new GetAllCountriesCommand();
            var result = await _mediator.Send(command);
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener países");
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    /// <summary>
    /// Obtiene un país por ID
    /// </summary>
    [HttpGet("{id}")]
    [ProducesResponseType(typeof(CountryDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<IActionResult> GetById(Guid id)
    {
        try
        {
            var command = new GetCountryByIdCommand(id);
            var result = await _mediator.Send(command);

            if (result == null)
                return NotFound(new { message = $"No se encontró el país con ID {id}" });

            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener país {Id}", id);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    /// <summary>
    /// Obtiene los estados/provincias de un país
    /// </summary>
    [HttpGet("{id}/states")]
    [ProducesResponseType(typeof(List<StateDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetStates(Guid id)
    {
        try
        {
            var command = new GetStatesByCountryIdCommand(id);
            var result = await _mediator.Send(command);
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener estados del país {Id}", id);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }
}
