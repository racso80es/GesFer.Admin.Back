using NetArchTest.Rules;
using Xunit;
using GesFer.Domain.Entities;
using System.Reflection;

namespace GesFer.Architecture.Tests;

public class TheWallTests
{
    [Fact]
    public void Domain_Should_Not_Depend_On_Infrastructure()
    {
        var domainAssembly = typeof(Article).Assembly;
        var result = Types.InAssembly(domainAssembly)
            .ShouldNot()
            .HaveDependencyOn("GesFer.Infrastructure")
            .GetResult();

        Assert.True(result.IsSuccessful, "GesFer.Domain should not depend on GesFer.Infrastructure.");
    }
}
