using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.Handlers.Geo;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Handlers.Geo;

public class GetAllCountriesHandlerTests
{
    private static AdminDbContext CreateContext()
    {
        var options = new DbContextOptionsBuilder<AdminDbContext>()
            .UseInMemoryDatabase(databaseName: Guid.NewGuid().ToString())
            .Options;
        return new AdminDbContext(options);
    }

    [Fact]
    public async Task Handle_ReturnsOnlyActiveCountries()
    {
        // Arrange
        await using var context = CreateContext();
        var inactiveId = Guid.NewGuid();
        context.Countries.AddRange(
            new Country { Id = Guid.NewGuid(), Name = "Spain", Code = "ES" },
            new Country { Id = Guid.NewGuid(), Name = "France", Code = "FR" },
            new Country { Id = inactiveId, Name = "Inactive", Code = "XX" }
        );
        await context.SaveChangesAsync();
        var inactive = await context.Countries.FindAsync([inactiveId], CancellationToken.None);
        inactive!.IsActive = false;
        inactive.DeletedAt = DateTime.UtcNow;
        await context.SaveChangesAsync();

        var handler = new GetAllCountriesHandler(context);

        // Act
        var result = await handler.Handle(new GetAllCountriesCommand(), CancellationToken.None);

        // Assert
        result.Should().HaveCount(2);
        result.Select(c => c.Name).Should().Contain(new[] { "Spain", "France" });
        result.Select(c => c.Name).Should().NotContain("Inactive");
    }
}
