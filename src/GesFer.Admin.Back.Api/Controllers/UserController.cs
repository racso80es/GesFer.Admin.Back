using GesFer.Admin.Back.Api.Attributes;
using GesFer.Admin.Back.Api.Security;
using GesFer.Admin.Back.Application.Commands.User;
using GesFer.Admin.Back.Application.DTOs.User;
using MediatR;
using Microsoft.AspNetCore.Mvc;

namespace GesFer.Admin.Back.Api.Controllers;

[ApiController]
[Route("api/[controller]")]
[AuthorizeSystemOrAdmin]
public class UserController : ControllerBase
{
    private readonly IMediator _mediator;
    private readonly ILogger<UserController> _logger;

    public UserController(IMediator mediator, ILogger<UserController> logger)
    {
        _mediator = mediator;
        _logger = logger;
    }

    [HttpGet]
    [ProducesResponseType(typeof(List<UserDto>), StatusCodes.Status200OK)]
    public async Task<IActionResult> GetAll([FromQuery] Guid? companyId = null)
    {
        try
        {
            if (!User.TryGetCompanyId(out var tokenCompanyId))
            {
                if (companyId == null)
                    return BadRequest(new { message = "CompanyId no presente en token; provee companyId en query para llamadas System." });
                tokenCompanyId = companyId.Value;
            }

            var result = await _mediator.Send(new GetAllUsersCommand(tokenCompanyId));
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener usuarios");
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    [HttpGet("{id}")]
    [ProducesResponseType(typeof(UserDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<IActionResult> GetById(Guid id, [FromQuery] Guid? companyId = null)
    {
        try
        {
            if (!User.TryGetCompanyId(out var tokenCompanyId))
            {
                if (companyId == null)
                    return BadRequest(new { message = "CompanyId no presente en token; provee companyId en query para llamadas System." });
                tokenCompanyId = companyId.Value;
            }

            var result = await _mediator.Send(new GetUserByIdCommand(id, tokenCompanyId));
            if (result == null)
                return NotFound(new { message = $"No se encontró el usuario con ID {id}" });
            return Ok(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al obtener usuario {Id}", id);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    [HttpPost]
    [ProducesResponseType(typeof(UserDto), StatusCodes.Status201Created)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<IActionResult> Create([FromBody] CreateUserDto dto)
    {
        try
        {
            var result = await _mediator.Send(new CreateUserCommand(dto));
            return CreatedAtAction(nameof(GetById), new { id = result.Id }, result);
        }
        catch (ArgumentException ex)
        {
            return BadRequest(new { message = ex.Message });
        }
        catch (InvalidOperationException ex)
        {
            return BadRequest(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al crear usuario");
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    [HttpPut("{id}")]
    [ProducesResponseType(typeof(UserDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<IActionResult> Update(Guid id, [FromBody] UpdateUserDto dto)
    {
        try
        {
            var result = await _mediator.Send(new UpdateUserCommand(id, dto));
            return Ok(result);
        }
        catch (ArgumentException ex)
        {
            return BadRequest(new { message = ex.Message });
        }
        catch (InvalidOperationException ex)
        {
            if (ex.Message.Contains("No se encontró"))
                return NotFound(new { message = ex.Message });
            return BadRequest(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al actualizar usuario {Id}", id);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }

    [HttpDelete("{id}")]
    [ProducesResponseType(StatusCodes.Status204NoContent)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<IActionResult> Delete(Guid id)
    {
        try
        {
            await _mediator.Send(new DeleteUserCommand(id));
            return NoContent();
        }
        catch (InvalidOperationException ex)
        {
            return NotFound(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al eliminar usuario {Id}", id);
            return StatusCode(500, new { message = "Error interno del servidor" });
        }
    }
}

