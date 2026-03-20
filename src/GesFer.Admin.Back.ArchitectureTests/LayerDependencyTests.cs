using System.Reflection;
using FluentAssertions;
using GesFer.Admin.Back.Application.Commands.Geo;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Api.Controllers;
using GesFer.Admin.Back.Infrastructure.Data;

namespace GesFer.Admin.Back.ArchitectureTests;

/// <summary>
/// Reglas mínimas de dependencia entre capas (reflexión; sin paquetes extra).
/// </summary>
public class LayerDependencyTests
{
    private static HashSet<string?> ReferencedAssemblyNames(Assembly assembly) =>
        assembly.GetReferencedAssemblies().Select(a => a.Name).ToHashSet(StringComparer.Ordinal);

    [Fact]
    public void Domain_must_not_reference_Application_Infrastructure_or_Api()
    {
        var asm = typeof(Country).Assembly;
        var refs = ReferencedAssemblyNames(asm);
        refs.Should().NotContain("GesFer.Admin.Back.Application");
        refs.Should().NotContain("GesFer.Admin.Back.Infrastructure");
        refs.Should().NotContain("GesFer.Admin.Back.Api");
    }

    [Fact]
    public void Application_must_not_reference_Infrastructure_or_Api()
    {
        var asm = typeof(GetAllCountriesCommand).Assembly;
        var refs = ReferencedAssemblyNames(asm);
        refs.Should().NotContain("GesFer.Admin.Back.Infrastructure");
        refs.Should().NotContain("GesFer.Admin.Back.Api");
    }

    [Fact]
    public void Infrastructure_must_not_reference_Api()
    {
        var asm = typeof(AdminDbContext).Assembly;
        var refs = ReferencedAssemblyNames(asm);
        refs.Should().NotContain("GesFer.Admin.Back.Api");
    }

    [Fact]
    public void Infrastructure_may_reference_Application_and_Domain()
    {
        var asm = typeof(AdminDbContext).Assembly;
        var refs = ReferencedAssemblyNames(asm);
        refs.Should().Contain("GesFer.Admin.Back.Application");
        refs.Should().Contain("GesFer.Admin.Back.Domain");
    }

    [Fact]
    public void Api_may_reference_Application_and_Infrastructure()
    {
        var asm = typeof(GeolocationController).Assembly;
        var refs = ReferencedAssemblyNames(asm);
        refs.Should().Contain("GesFer.Admin.Back.Application");
        refs.Should().Contain("GesFer.Admin.Back.Infrastructure");
    }
}
