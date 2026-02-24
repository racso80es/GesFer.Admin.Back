using FluentAssertions;
using GesFer.Admin.Back.Domain.ValueObjects;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Domain.ValueObjects;

public class ActionStatusTests
{
    [Theory]
    [InlineData("Pending")]
    [InlineData("Running")]
    [InlineData("Completed")]
    [InlineData("Failed")]
    [InlineData("Cancelled")]
    public void Create_ShouldReturnActionStatus_WhenValueIsValid(string value)
    {
        var status = ActionStatus.Create(value);
        status.Value.Should().Be(value);
    }

    [Fact]
    public void Create_ShouldThrowArgumentException_WhenValueIsInvalid()
    {
        Action act = () => ActionStatus.Create("UnknownStatus");
        act.Should().Throw<ArgumentException>().WithMessage("Estado de acción inválido: UnknownStatus*");
    }

    [Fact]
    public void TryCreate_ShouldReturnFalse_WhenValueIsInvalid()
    {
        var result = ActionStatus.TryCreate("Invalid", out var status);
        result.Should().BeFalse();
        status.Value.Should().BeNull();
    }

    [Fact]
    public void Equals_ShouldReturnTrue_WhenValuesAreEqual()
    {
        var status1 = ActionStatus.Create("Pending");
        var status2 = ActionStatus.Create("Pending");
        status1.Should().Be(status2);
        (status1 == status2).Should().BeTrue();
    }
}
