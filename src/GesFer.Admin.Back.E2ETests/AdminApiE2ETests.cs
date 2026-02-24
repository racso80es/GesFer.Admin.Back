using System.Net;
using System.Net.Http.Json;
using FluentAssertions;
using GesFer.Admin.Back.Application.DTOs.Company;
using GesFer.Admin.Back.Application.DTOs.Geo;
using Xunit;

namespace GesFer.Admin.Back.E2ETests;

/// <summary>
/// Pruebas E2E contra la API Admin en local: infra en punto de origen, datos y servicios preparados
/// (prepare-full-env + invoke-mysql-seeds + API en marcha). Ejecutar con Run-E2ELocal.ps1 o
/// manualmente con E2E_BASE_URL opcional.
/// </summary>
[Trait("Category", "E2E")]
public class AdminApiE2ETests : IClassFixture<E2EFixture>
{
    private readonly E2EFixture _fixture;

    public AdminApiE2ETests(E2EFixture fixture) => _fixture = fixture;

    [Fact]
    public async Task Health_Returns_200()
    {
        using var client = E2EContext.CreateClient();
        var response = await client.GetAsync("/health");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
    }

    [Fact]
    public async Task Swagger_Json_Returns_200_And_Json()
    {
        using var client = E2EContext.CreateClient();
        var response = await client.GetAsync("/swagger/v1/swagger.json");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        response.Content.Headers.ContentType?.MediaType.Should().Be("application/json");
        var body = await response.Content.ReadAsStringAsync();
        body.Should().NotBeNullOrEmpty();
        body.TrimStart().Should().StartWith("{");
    }

    [Fact]
    public async Task Countries_WithInternalSecret_Returns_200_And_List()
    {
        using var client = E2EContext.CreateClient(withInternalSecret: true);
        var response = await client.GetAsync("/api/countries");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CountryDto>>();
        list.Should().NotBeNull();
        list!.Count.Should().BeGreaterThan(0);
        list.Should().Contain(c => c.Name == "España");
    }

    [Fact]
    public async Task Company_WithInternalSecret_Returns_200_And_List()
    {
        using var client = E2EContext.CreateClient(withInternalSecret: true);
        var response = await client.GetAsync("/api/company");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CompanyDto>>();
        list.Should().NotBeNull();
        list!.Count.Should().BeGreaterThan(0);
    }

    [Fact]
    public async Task Admin_Login_Returns_200_And_Token()
    {
        using var client = E2EContext.CreateClient();
        var response = await client.PostAsJsonAsync("/api/admin/auth/login", new
        {
            Usuario = E2EContext.AdminUser,
            Contraseña = E2EContext.AdminPassword
        });
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var body = await response.Content.ReadAsStringAsync();
        body.Should().Contain("Token");
        body.Should().Contain("Admin");
    }

    [Fact]
    public async Task Company_WithAdminJwt_Returns_200()
    {
        using var client = E2EContext.CreateClient();
        var loginResponse = await client.PostAsJsonAsync("/api/admin/auth/login", new
        {
            Usuario = E2EContext.AdminUser,
            Contraseña = E2EContext.AdminPassword
        });
        loginResponse.EnsureSuccessStatusCode();
        var login = await loginResponse.Content.ReadFromJsonAsync<AdminLoginResponse>();
        login.Should().NotBeNull();
        login!.Token.Should().NotBeNullOrEmpty();

        using var apiClient = E2EContext.CreateClient();
        apiClient.DefaultRequestHeaders.Authorization =
            new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", login.Token);
        var response = await apiClient.GetAsync("/api/company");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CompanyDto>>();
        list.Should().NotBeNull();
    }

    private sealed class AdminLoginResponse
    {
        public string Token { get; set; } = string.Empty;
    }
}
