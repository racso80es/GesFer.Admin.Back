using System.Net;
using System.Text.Json;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Api.Middleware;

public class GlobalExceptionHandlingMiddleware
{
    private readonly RequestDelegate _next;
    private readonly ILogger<GlobalExceptionHandlingMiddleware> _logger;

    public GlobalExceptionHandlingMiddleware(RequestDelegate next, ILogger<GlobalExceptionHandlingMiddleware> logger)
    {
        _next = next;
        _logger = logger;
    }

    public async Task InvokeAsync(HttpContext context)
    {
        try
        {
            await _next(context);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Excepción no manejada atrapada en el middleware global.");
            await HandleExceptionAsync(context, ex);
        }
    }

    private static Task HandleExceptionAsync(HttpContext context, Exception exception)
    {
        context.Response.ContentType = "application/json";

        var statusCode = (int)HttpStatusCode.InternalServerError;
        var message = "Error interno del servidor.";

        if (exception is ArgumentException)
        {
            statusCode = (int)HttpStatusCode.BadRequest;
            message = exception.Message;
        }
        else if (exception is InvalidOperationException invalidOpEx)
        {
            if (invalidOpEx.Message.Contains("No encontró", StringComparison.OrdinalIgnoreCase))
            {
                statusCode = (int)HttpStatusCode.NotFound;
            }
            else
            {
                statusCode = (int)HttpStatusCode.BadRequest;
            }
            message = invalidOpEx.Message;
        }

        context.Response.StatusCode = statusCode;

        var result = JsonSerializer.Serialize(new { error = message });
        return context.Response.WriteAsync(result);
    }
}
