using GesFer.Admin.Back.Application.DTOs.Auth;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using System.Net.Http.Json;
using Xunit;
using FluentAssertions;

namespace GesFer.Admin.Back.IntegrationTests;

[Collection("AdminIntegrationTests")]
public class AdminAuthIntegrationTests
{
    private readonly AdminWebAppFactory _factory;
    private readonly HttpClient _client;

    public AdminAuthIntegrationTests(AdminWebAppFactory factory)
    {
        _factory = factory;
        _client = factory.CreateClient();
    }

    [Fact]
    public async Task AdminLogin_WithDefaultCredentials_CreatesAuditLogRecord()
    {
        // Arrange
        var request = new AdminLoginRequest
        {
            Usuario = "admin",
            Contraseña = "admin123"
        };

        // Act
        var response = await _client.PostAsJsonAsync("/api/admin/auth/login", request);

        // Assert
        response.EnsureSuccessStatusCode();

        using var scope = _factory.Services.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();
        var auditLogs = await context.AuditLogs
            .Where(a => a.Action == "LoginSuccess" && a.Username == "admin")
            .ToListAsync();

        auditLogs.Should().NotBeEmpty("el login exitoso debe registrar un AuditLog con Action=LoginSuccess");
        var log = auditLogs.OrderByDescending(a => a.ActionTimestamp).First();
        log.CursorId.Should().NotBeNullOrEmpty();
        log.Path.Should().Be("/api/admin/auth/login");
        log.HttpMethod.Should().Be("POST");
    }

    [Fact]
    public async Task AdminLogin_WithDefaultCredentials_ReturnsToken()
    {
        // Arrange
        var request = new AdminLoginRequest
        {
            Usuario = "admin",
            Contraseña = "admin123"
        };

        // Act
        var response = await _client.PostAsJsonAsync("/api/admin/auth/login", request);

        // Assert
        response.EnsureSuccessStatusCode();
        var result = await response.Content.ReadFromJsonAsync<AdminLoginResponse>();

        result.Should().NotBeNull();
        result!.Token.Should().NotBeNullOrEmpty();
        result.Role.Should().Be("Admin");
        result.Username.Should().Be("admin");
    }

    [Fact]
    public async Task AdminLogin_WithInvalidPassword_ReturnsUnauthorized()
    {
        // Arrange
        var request = new AdminLoginRequest
        {
            Usuario = "admin",
            Contraseña = "wrongpassword"
        };

        // Act
        var response = await _client.PostAsJsonAsync("/api/admin/auth/login", request);

        // Assert
        response.StatusCode.Should().Be(System.Net.HttpStatusCode.Unauthorized);
    }

    /// <summary>
    /// Confirma que un login fallido inserta un registro LoginFailed en AuditLogs.
    /// </summary>
    [Fact]
    public async Task AdminLogin_WithInvalidCredentials_InsertsLoginFailedAuditLog()
    {
        // Arrange
        var request = new AdminLoginRequest
        {
            Usuario = "admin",
            Contraseña = "wrongpassword"
        };

        // Act
        var response = await _client.PostAsJsonAsync("/api/admin/auth/login", request);

        // Assert - respuesta 401
        response.StatusCode.Should().Be(System.Net.HttpStatusCode.Unauthorized);

        // Assert - inserción en AuditLogs
        using var scope = _factory.Services.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();
        var auditLogs = await context.AuditLogs
            .Where(a => a.Action == "LoginFailed" && a.Username == "admin")
            .ToListAsync();

        auditLogs.Should().NotBeEmpty("el login fallido debe registrar un AuditLog con Action=LoginFailed");
        var log = auditLogs.OrderByDescending(a => a.ActionTimestamp).First();
        log.CursorId.Should().BeEmpty("LoginFailed no tiene CursorId");
        log.Path.Should().Be("/api/admin/auth/login");
        log.HttpMethod.Should().Be("POST");
    }
}
