using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.Handlers.Geo;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Handlers.Geo;

public class GetCitiesByStateIdHandlerTests
{
    private static AdminDbContext CreateContext()
    {
        var options = new DbContextOptionsBuilder<AdminDbContext>()
            .UseInMemoryDatabase(databaseName: Guid.NewGuid().ToString())
            .Options;
        return new AdminDbContext(options);
    }

    [Fact]
    public async Task Handle_ReturnsCitiesForState()
    {
        // Arrange
        var stateId = Guid.NewGuid();
        await using var context = CreateContext();
        context.Cities.AddRange(
            new City { Id = Guid.NewGuid(), StateId = stateId, Name = "Madrid City" },
            new City { Id = Guid.NewGuid(), StateId = stateId, Name = "Alcala" },
            new City { Id = Guid.NewGuid(), StateId = Guid.NewGuid(), Name = "Other City" } // Other state
        );
        await context.SaveChangesAsync();

        var handler = new GetCitiesByStateIdHandler(context);

        // Act
        var result = await handler.Handle(new GetCitiesByStateIdCommand(stateId), CancellationToken.None);

        // Assert
        result.Should().HaveCount(2);
        result.Select(c => c.Name).Should().Contain(new[] { "Madrid City", "Alcala" });
    }

    [Fact]
    public async Task Handle_ExcludesInactiveCities()
    {
        var stateId = Guid.NewGuid();
        await using var context = CreateContext();
        var offId = Guid.NewGuid();
        context.Cities.AddRange(
            new City { Id = Guid.NewGuid(), StateId = stateId, Name = "On" },
            new City { Id = offId, StateId = stateId, Name = "Off" }
        );
        await context.SaveChangesAsync();
        var off = await context.Cities.FindAsync([offId], CancellationToken.None);
        off!.IsActive = false;
        await context.SaveChangesAsync();

        var handler = new GetCitiesByStateIdHandler(context);
        var result = await handler.Handle(new GetCitiesByStateIdCommand(stateId), CancellationToken.None);

        result.Should().HaveCount(1);
        result[0].Name.Should().Be("On");
    }
}
