using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Application.Handlers.Geo;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Handlers.Geo;

public class GetPostalCodesByCityIdHandlerTests
{
    private static AdminDbContext CreateContext()
    {
        var options = new DbContextOptionsBuilder<AdminDbContext>()
            .UseInMemoryDatabase(databaseName: Guid.NewGuid().ToString())
            .Options;
        return new AdminDbContext(options);
    }

    [Fact]
    public async Task Handle_ReturnsPostalCodesForCity_OrderedByCode()
    {
        var cityId = Guid.NewGuid();
        await using var context = CreateContext();
        context.PostalCodes.AddRange(
            new PostalCode { Id = Guid.NewGuid(), CityId = cityId, Code = "28002", IsActive = true },
            new PostalCode { Id = Guid.NewGuid(), CityId = cityId, Code = "28001", IsActive = true },
            new PostalCode { Id = Guid.NewGuid(), CityId = Guid.NewGuid(), Code = "99999", IsActive = true }
        );
        await context.SaveChangesAsync();

        var handler = new GetPostalCodesByCityIdHandler(context);

        var result = await handler.Handle(new GetPostalCodesByCityIdCommand(cityId), CancellationToken.None);

        result.Should().HaveCount(2);
        result.Select(p => p.Code).Should().ContainInOrder("28001", "28002");
    }

    [Fact]
    public async Task Handle_ExcludesInactivePostalCodes()
    {
        var cityId = Guid.NewGuid();
        await using var context = CreateContext();
        var offId = Guid.NewGuid();
        context.PostalCodes.AddRange(
            new PostalCode { Id = Guid.NewGuid(), CityId = cityId, Code = "11111", IsActive = true },
            new PostalCode { Id = offId, CityId = cityId, Code = "22222", IsActive = true }
        );
        await context.SaveChangesAsync();
        var off = await context.PostalCodes.FindAsync([offId], CancellationToken.None);
        off!.IsActive = false;
        await context.SaveChangesAsync();

        var handler = new GetPostalCodesByCityIdHandler(context);
        var result = await handler.Handle(new GetPostalCodesByCityIdCommand(cityId), CancellationToken.None);

        result.Should().HaveCount(1);
        result.First().Code.Should().Be("11111");
    }
}
