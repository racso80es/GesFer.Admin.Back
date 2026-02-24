using FluentAssertions;
using GesFer.Admin.Back.Domain.ValueObjects;
using Xunit;

namespace GesFer.Admin.Back.UnitTests.Domain.ValueObjects;

public class ProcessIdTests
{
    [Fact]
    public void Create_ShouldReturnProcessId_WhenGuidIsValid()
    {
        var guid = Guid.NewGuid();
        var processId = ProcessId.Create(guid);
        processId.Value.Should().Be(guid);
    }

    [Fact]
    public void Create_ShouldThrowArgumentException_WhenGuidIsEmpty()
    {
        Action act = () => ProcessId.Create(Guid.Empty);
        act.Should().Throw<ArgumentException>().WithMessage("El ID del proceso no puede ser Guid.Empty.*");
    }

    [Fact]
    public void Create_FromString_ShouldReturnProcessId_WhenStringIsValid()
    {
        var guid = Guid.NewGuid();
        var processId = ProcessId.Create(guid.ToString());
        processId.Value.Should().Be(guid);
    }

    [Fact]
    public void TryCreate_ShouldReturnFalse_WhenStringIsInvalid()
    {
        var result = ProcessId.TryCreate("invalid-guid", out var processId);
        result.Should().BeFalse();
        processId.Value.Should().Be(Guid.Empty);
    }

    [Fact]
    public void Equals_ShouldReturnTrue_WhenValuesAreEqual()
    {
        var guid = Guid.NewGuid();
        var id1 = ProcessId.Create(guid);
        var id2 = ProcessId.Create(guid);
        id1.Should().Be(id2);
        (id1 == id2).Should().BeTrue();
    }
}
