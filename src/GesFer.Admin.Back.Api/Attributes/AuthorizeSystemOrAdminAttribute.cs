using GesFer.Admin.Back.Api.Configuration;
using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.Filters;
using Microsoft.Extensions.Options;

namespace GesFer.Admin.Back.Api.Attributes;

[AttributeUsage(AttributeTargets.Class | AttributeTargets.Method)]
public class AuthorizeSystemOrAdminAttribute : TypeFilterAttribute
{
    public AuthorizeSystemOrAdminAttribute() : base(typeof(AuthorizeSystemOrAdminFilter))
    {
    }
}

public class AuthorizeSystemOrAdminFilter : IAsyncAuthorizationFilter
{
    private readonly ILogger<AuthorizeSystemOrAdminFilter> _logger;
    private readonly SystemAuthOptions _options;

    public AuthorizeSystemOrAdminFilter(ILogger<AuthorizeSystemOrAdminFilter> logger, IOptions<SystemAuthOptions> options)
    {
        _logger = logger;
        _options = options.Value;
    }

    public Task OnAuthorizationAsync(AuthorizationFilterContext context)
    {
        var httpContext = context.HttpContext;

        // 1. Validar Shared Secret (System)
        if (httpContext.Request.Headers.TryGetValue("X-Internal-Secret", out var secretValues))
        {
            var secret = secretValues.FirstOrDefault();
            var configuredSecret = _options.SharedSecret;

            if (!string.IsNullOrEmpty(secret) && secret == configuredSecret)
            {
                // Autenticado como Sistema
                return Task.CompletedTask;
            }
            _logger.LogWarning("Intento de acceso por sistema fallido: Secret incorrecto");
        }

        // 2. Validar JWT (Admin) - Si no es sistema, delegar a la autenticación estándar de ASP.NET Core
        // Pero como este es un filtro personalizado, necesitamos verificar manualmente si el usuario está autenticado y es Admin
        // O simplemente dejar pasar si no hay secret, asumiendo que el endpoint tiene [Authorize] adicional.
        // Sin embargo, para simplificar, si no es sistema, requerimos que el usuario esté autenticado como Admin.

        var user = httpContext.User;
        if (user.Identity?.IsAuthenticated == true && user.IsInRole("Admin"))
        {
            // Autenticado como Admin
            return Task.CompletedTask;
        }

        // Fallo en ambas
        context.Result = new UnauthorizedResult();
        return Task.CompletedTask;
    }
}
