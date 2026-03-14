using System;
using System.Threading.Tasks;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Moq;
using GesFer.Admin.Back.Infrastructure.Services;
using Xunit;
using FluentAssertions;

namespace GesFer.Admin.Back.UnitTests.Services;

public class AuditLogServiceTests
{
    private readonly string _dbName;
    private readonly IServiceProvider _serviceProvider;
    private readonly Mock<ILogger<AuditLogService>> _loggerMock;
    private readonly AuditLogService _auditLogService;

    public AuditLogServiceTests()
    {
        _dbName = Guid.NewGuid().ToString();
        var services = new ServiceCollection();
        services.AddDbContext<AdminDbContext>(opts => opts.UseInMemoryDatabase(_dbName));
        _serviceProvider = services.BuildServiceProvider();
        _loggerMock = new Mock<ILogger<AuditLogService>>();
        _auditLogService = new AuditLogService(_serviceProvider, _loggerMock.Object);
    }

    [Fact]
    public async Task LogActionAsync_ShouldCreateAuditLog_WhenCalledValidly()
    {
        // Arrange
        var cursorId = "user-123";
        var username = "testuser";
        var action = "TestResult";
        var httpMethod = "GET";
        var path = "/api/test";
        var additionalData = "{ \"key\": \"value\" }";

        // Act
        await _auditLogService.LogActionAsync(cursorId, username, action, httpMethod, path, additionalData);

        // Assert - usar el mismo DbContext (misma BD InMemory) que el servicio
        using var scope = _serviceProvider.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();
        var log = await context.AuditLogs.FirstOrDefaultAsync();
        log.Should().NotBeNull();
        log!.CursorId.Should().Be(cursorId);
        log.Username.Should().Be(username);
        log.Action.Should().Be(action);
        log.HttpMethod.Should().Be(httpMethod);
        log.Path.Should().Be(path);
        log.AdditionalData.Should().Be(additionalData);
        // KZ-BACK-002: Usar aserción de rango para evitar flakiness en tests de tiempo
        log.ActionTimestamp.Should().BeCloseTo(DateTime.UtcNow, TimeSpan.FromSeconds(2));
        log.IsActive.Should().BeTrue();
    }

    [Fact]
    public async Task LogActionAsync_ShouldNotThrow_WhenDatabaseFails()
    {
        // Arrange - ServiceProvider que devuelve un DbContext ya disposed para simular fallo de BD
        var failDbName = "fail-" + Guid.NewGuid();
        var failServices = new ServiceCollection();
        failServices.AddScoped<AdminDbContext>(_ =>
        {
            var opts = new DbContextOptionsBuilder<AdminDbContext>()
                .UseInMemoryDatabase(failDbName)
                .Options;
            var ctx = new AdminDbContext(opts);
            ctx.Dispose();
            return ctx;
        });
        var failProvider = failServices.BuildServiceProvider();
        var failService = new AuditLogService(failProvider, _loggerMock.Object);

        // Act
        Func<Task> act = async () => await failService.LogActionAsync("cursor", "user", "action", "POST", "/path", null);

        // Assert
        await act.Should().NotThrowAsync();

        // Verify logger was called
        _loggerMock.Verify(x => x.Log(
            LogLevel.Error,
            It.IsAny<EventId>(),
            It.Is<It.IsAnyType>((v, t) => true),
            It.IsAny<Exception>(),
            It.Is<Func<It.IsAnyType, Exception?, string>>((v, t) => true)), Times.Once);
    }

    [Fact]
    public async Task LogActionAsync_ShouldHandleNullAdditionalData_AndSetIsActive()
    {
        // Act
        await _auditLogService.LogActionAsync("cursor", "user", "action", "POST", "/path", null);

        // Assert
        using var scope = _serviceProvider.CreateScope();
        var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();
        var log = await context.AuditLogs.FirstAsync();
        log.Should().NotBeNull();
        log.AdditionalData.Should().BeNull();
        log.IsActive.Should().BeTrue();
    }
}
