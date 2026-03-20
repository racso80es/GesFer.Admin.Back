using GesFer.Admin.Back.Api.Attributes;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.DTOs.Geo;
using MediatR;
using Microsoft.AspNetCore.Mvc;

namespace GesFer.Admin.Back.Api.Controllers;

/// <summary>
/// Lectura unificada de jerarquía geográfica (país → estado → ciudad → código postal).
/// </summary>
[ApiController]
[Route("api/geolocation")]
[AuthorizeSystemOrAdmin]
public class GeolocationController : ControllerBase
{
    private readonly IMediator _mediator;
    private readonly ILogger<GeolocationController> _logger;

    public GeolocationController(IMediator mediator, ILogger<GeolocationController> logger)
    {
        _mediator = mediator;
        _logger = logger;
    }

    /// <summary>Obtiene todos los países activos.</summary>
    [HttpGet("countries")]
    [ProducesResponseType(typeof(List<CountryGeoReadDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetCountries()
    {
        try
        {
            var result = await _mediator.Send(new GetAllCountriesCommand());
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener países");
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    /// <summary>Obtiene un país por id (solo si está activo).</summary>
    [HttpGet("countries/{countryId:guid}")]
    [ProducesResponseType(typeof(CountryGeoReadDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<IActionResult> GetCountryById(Guid countryId)
    {
        try
        {
            var result = await _mediator.Send(new GetCountryByIdCommand(countryId));
            if (result == null)
                return NotFound(new { message = $"No se encontró el país con ID {countryId}" });
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener país {CountryId}", countryId);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    /// <summary>Estados/provincias de un país.</summary>
    [HttpGet("countries/{countryId:guid}/states")]
    [ProducesResponseType(typeof(List<StateGeoReadDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetStatesByCountry(Guid countryId)
    {
        try
        {
            var result = await _mediator.Send(new GetStatesByCountryIdCommand(countryId));
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error states país {CountryId}", countryId);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    /// <summary>Ciudades de un estado.</summary>
    [HttpGet("states/{stateId:guid}/cities")]
    [ProducesResponseType(typeof(List<CityGeoReadDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetCitiesByState(Guid stateId)
    {
        try
        {
            var result = await _mediator.Send(new GetCitiesByStateIdCommand(stateId));
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error ciudades estado {StateId}", stateId);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    /// <summary>Códigos postales de una ciudad.</summary>
    [HttpGet("cities/{cityId:guid}/postal-codes")]
    [ProducesResponseType(typeof(List<PostalCodeGeoReadDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetPostalCodesByCity(Guid cityId)
    {
        try
        {
            var result = await _mediator.Send(new GetPostalCodesByCityIdCommand(cityId));
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error códigos postales ciudad {CityId}", cityId);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }
}
