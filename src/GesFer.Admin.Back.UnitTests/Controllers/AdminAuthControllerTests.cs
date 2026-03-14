using FluentAssertions;
using GesFer.Admin.Back.Api.Controllers;
using GesFer.Admin.Back.Application.Commands.Auth;
using GesFer.Admin.Back.Application.DTOs.Auth;
using MediatR;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using Moq;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Controllers;

public class AdminAuthControllerTests
{
    private readonly Mock<ISender> _mockSender;
    private readonly Mock<ILogger<AdminAuthController>> _mockLogger;
    private readonly AdminAuthController _controller;

    public AdminAuthControllerTests()
    {
        _mockSender = new Mock<ISender>();
        _mockLogger = new Mock<ILogger<AdminAuthController>>();
        _controller = new AdminAuthController(_mockSender.Object, _mockLogger.Object)
        {
            ControllerContext = new ControllerContext { HttpContext = new DefaultHttpContext() }
        };
    }

    [Fact]
    public async Task Login_WithEmptyCredentials_ShouldReturnBadRequest()
    {
        // Arrange
        var request = new AdminLoginRequest { Usuario = "", Contraseña = "" };
        _mockSender.Setup(s => s.Send(It.IsAny<AdminLoginCommand>(), default))
            .ReturnsAsync(AdminLoginResult.ValidationError("Usuario y contraseña son requeridos"));

        // Act
        var result = await _controller.Login(request);

        // Assert
        result.Should().BeOfType<BadRequestObjectResult>()
            .Which.Value.Should().BeEquivalentTo(new { message = "Usuario y contraseña son requeridos" });
    }

    [Fact]
    public async Task Login_WithInvalidCredentials_ShouldReturnUnauthorized()
    {
        // Arrange
        var request = new AdminLoginRequest { Usuario = "invalid", Contraseña = "wrongpassword" };
        _mockSender.Setup(s => s.Send(It.IsAny<AdminLoginCommand>(), default))
            .ReturnsAsync(AdminLoginResult.AuthFailure("Credenciales administrativas inválidas"));

        // Act
        var result = await _controller.Login(request);

        // Assert
        result.Should().BeOfType<UnauthorizedObjectResult>()
            .Which.Value.Should().BeEquivalentTo(new { message = "Credenciales administrativas inválidas" });
    }

    [Fact]
    public async Task Login_WithValidCredentials_ShouldReturnOk()
    {
        // Arrange
        var request = new AdminLoginRequest { Usuario = "admin", Contraseña = "password" };
        var userId = Guid.NewGuid();
        var response = new AdminLoginResponse
        {
            UserId = userId.ToString(),
            CursorId = userId.ToString(),
            Username = "admin",
            FirstName = "Admin",
            LastName = "User",
            Email = "admin@example.com",
            Role = "Admin",
            Token = "generated-token"
        };
        _mockSender.Setup(s => s.Send(It.IsAny<AdminLoginCommand>(), default))
            .ReturnsAsync(AdminLoginResult.Success(response));

        // Act
        var result = await _controller.Login(request);

        // Assert
        var okResult = result.Should().BeOfType<OkObjectResult>().Subject;
        var actualResponse = okResult.Value.Should().BeOfType<AdminLoginResponse>().Subject;

        actualResponse.Should().NotBeNull();
        actualResponse.Username.Should().Be("admin");
        actualResponse.Token.Should().Be("generated-token");
        actualResponse.UserId.Should().Be(userId.ToString());
    }

    [Fact]
    public async Task Login_WhenExceptionOccurs_ShouldReturnInternalServerError()
    {
        // Arrange
        var request = new AdminLoginRequest { Usuario = "admin", Contraseña = "password" };
        _mockSender.Setup(s => s.Send(It.IsAny<AdminLoginCommand>(), default))
            .ThrowsAsync(new Exception("Database error"));

        // Act
        var result = await _controller.Login(request);

        // Assert
        var objectResult = result.Should().BeOfType<ObjectResult>().Subject;
        objectResult.StatusCode.Should().Be(500);
        objectResult.Value.Should().BeEquivalentTo(new { message = "Error interno del servidor", error = "Database error" });
    }
}
