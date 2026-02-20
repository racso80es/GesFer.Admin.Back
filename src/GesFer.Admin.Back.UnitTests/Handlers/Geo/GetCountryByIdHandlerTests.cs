using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.Handlers.Geo;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Handlers.Geo;

public class GetCountryByIdHandlerTests
{
    private static AdminDbContext CreateContext()
    {
        var options = new DbContextOptionsBuilder<AdminDbContext>()
            .UseInMemoryDatabase(databaseName: Guid.NewGuid().ToString())
            .Options;
        return new AdminDbContext(options);
    }

    [Fact]
    public async Task Handle_ReturnsCountry_WhenFoundAndActive()
    {
        // Arrange
        var id = Guid.NewGuid();
        await using var context = CreateContext();
        context.Countries.Add(new Country { Id = id, Name = "Spain", Code = "ES", IsActive = true });
        await context.SaveChangesAsync();

        var handler = new GetCountryByIdHandler(context);

        // Act
        var result = await handler.Handle(new GetCountryByIdCommand(id), CancellationToken.None);

        // Assert
        result.Should().NotBeNull();
        result!.Id.Should().Be(id);
        result.Name.Should().Be("Spain");
    }

    [Fact]
    public async Task Handle_ReturnsNull_WhenDeleted()
    {
        // Arrange
        var id = Guid.NewGuid();
        await using var context = CreateContext();
        context.Countries.Add(new Country { Id = id, Name = "Deleted", Code = "XX", DeletedAt = DateTime.UtcNow });
        await context.SaveChangesAsync();

        var handler = new GetCountryByIdHandler(context);

        // Act
        var result = await handler.Handle(new GetCountryByIdCommand(id), CancellationToken.None);

        // Assert
        result.Should().BeNull();
    }
}
