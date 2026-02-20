using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.Handlers.Geo;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Handlers.Geo;

public class GetStatesByCountryIdHandlerTests
{
    private static AdminDbContext CreateContext()
    {
        var options = new DbContextOptionsBuilder<AdminDbContext>()
            .UseInMemoryDatabase(databaseName: Guid.NewGuid().ToString())
            .Options;
        return new AdminDbContext(options);
    }

    [Fact]
    public async Task Handle_ReturnsStatesForCountry()
    {
        // Arrange
        var countryId = Guid.NewGuid();
        await using var context = CreateContext();
        context.States.AddRange(
            new State { Id = Guid.NewGuid(), CountryId = countryId, Name = "Madrid", Code = "MD" },
            new State { Id = Guid.NewGuid(), CountryId = countryId, Name = "Barcelona", Code = "BCN" },
            new State { Id = Guid.NewGuid(), CountryId = Guid.NewGuid(), Name = "Other", Code = "OT" } // Other country
        );
        await context.SaveChangesAsync();

        var handler = new GetStatesByCountryIdHandler(context);

        // Act
        var result = await handler.Handle(new GetStatesByCountryIdCommand(countryId), CancellationToken.None);

        // Assert
        result.Should().HaveCount(2);
        result.Select(s => s.Name).Should().Contain(new[] { "Madrid", "Barcelona" });
    }
}
