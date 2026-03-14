using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Auth;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Auth;
using GesFer.Admin.Back.Application.Handlers.Auth;
using GesFer.Admin.Back.Domain.Entities;
using Microsoft.Extensions.Logging;
using Moq;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Handlers;

public class AdminLoginHandlerTests
{
    private readonly Mock<IAdminAuthService> _mockAuthService;
    private readonly Mock<IAdminJwtService> _mockJwtService;
    private readonly Mock<IAuditLogService> _mockAuditService;
    private readonly Mock<ILogger<AdminLoginHandler>> _mockLogger;
    private readonly AdminLoginHandler _handler;

    public AdminLoginHandlerTests()
    {
        _mockAuthService = new Mock<IAdminAuthService>();
        _mockJwtService = new Mock<IAdminJwtService>();
        _mockAuditService = new Mock<IAuditLogService>();
        _mockLogger = new Mock<ILogger<AdminLoginHandler>>();
        _handler = new AdminLoginHandler(
            _mockAuthService.Object,
            _mockJwtService.Object,
            _mockAuditService.Object,
            _mockLogger.Object);
    }

    [Fact]
    public async Task Handle_WithEmptyUsuario_ShouldReturnValidationError()
    {
        var command = new AdminLoginCommand("", "password", null, null);

        var result = await _handler.Handle(command, default);

        result.IsSuccess.Should().BeFalse();
        result.HttpStatusCode.Should().Be(400);
        result.ErrorMessage.Should().Be("Usuario y contraseña son requeridos");
        _mockAuthService.Verify(s => s.AuthenticateAsync(It.IsAny<string>(), It.IsAny<string>()), Times.Never);
        _mockAuditService.Verify(s => s.LogActionAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string?>()), Times.Never);
    }

    [Fact]
    public async Task Handle_WithEmptyContraseña_ShouldReturnValidationError()
    {
        var command = new AdminLoginCommand("admin", "", null, null);

        var result = await _handler.Handle(command, default);

        result.IsSuccess.Should().BeFalse();
        result.HttpStatusCode.Should().Be(400);
        result.ErrorMessage.Should().Be("Usuario y contraseña son requeridos");
        _mockAuthService.Verify(s => s.AuthenticateAsync(It.IsAny<string>(), It.IsAny<string>()), Times.Never);
    }

    [Fact]
    public async Task Handle_WithInvalidCredentials_ShouldReturnAuthFailure_AndLogAudit()
    {
        var command = new AdminLoginCommand("invalid", "wrong", "127.0.0.1", "Mozilla/5.0");
        _mockAuthService.Setup(s => s.AuthenticateAsync("invalid", "wrong"))
            .ReturnsAsync((AdminUser?)null);

        var result = await _handler.Handle(command, default);

        result.IsSuccess.Should().BeFalse();
        result.HttpStatusCode.Should().Be(401);
        result.ErrorMessage.Should().Be("Credenciales administrativas inválidas");
        _mockAuditService.Verify(s => s.LogActionAsync(
            "",
            "invalid",
            "LoginFailed",
            "POST",
            "/api/admin/auth/login",
            It.Is<string>(d => d != null && d.Contains("127.0.0.1") && d.Contains("Mozilla"))), Times.Once);
    }

    [Fact]
    public async Task Handle_WithValidCredentials_ShouldReturnSuccess_AndLogAudit()
    {
        var userId = Guid.NewGuid();
        var adminUser = new AdminUser
        {
            Id = userId,
            Username = "admin",
            FirstName = "Admin",
            LastName = "User",
            Email = "admin@example.com"
        };
        var command = new AdminLoginCommand("admin", "admin123", "192.168.1.1", "Postman");
        _mockAuthService.Setup(s => s.AuthenticateAsync("admin", "admin123"))
            .ReturnsAsync(adminUser);
        _mockJwtService.Setup(s => s.GenerateAdminToken(userId.ToString(), "admin", userId))
            .Returns("jwt-token");

        var result = await _handler.Handle(command, default);

        result.IsSuccess.Should().BeTrue();
        result.Response.Should().NotBeNull();
        result.Response!.Username.Should().Be("admin");
        result.Response.Token.Should().Be("jwt-token");
        result.Response.UserId.Should().Be(userId.ToString());
        _mockAuditService.Verify(s => s.LogActionAsync(
            userId.ToString(),
            "admin",
            "LoginSuccess",
            "POST",
            "/api/admin/auth/login",
            It.Is<string>(d => d != null && d.Contains("192.168.1.1") && d.Contains("Postman"))), Times.Once);
    }
}
