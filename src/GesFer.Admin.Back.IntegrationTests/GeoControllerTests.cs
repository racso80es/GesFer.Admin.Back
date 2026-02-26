using System.Net;
using System.Net.Http.Json;
using FluentAssertions;
using GesFer.Admin.Back.Application.DTOs.Geo;
using Xunit;

namespace GesFer.Admin.Back.IntegrationTests;

[Collection("AdminIntegrationTests")]
public class GeoControllerTests
{
    private const string InternalSecret = "test-internal-secret";
    private readonly AdminWebAppFactory _factory;
    private readonly HttpClient _client;

    public GeoControllerTests(AdminWebAppFactory factory)
    {
        _factory = factory;
        _client = factory.CreateClient();
        _client.DefaultRequestHeaders.Add("X-Internal-Secret", InternalSecret);
    }

    [Fact]
    public async Task GetAllCountries_ShouldReturnList()
    {
        var response = await _client.GetAsync("/api/countries");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CountryDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(c => c.Name == "España");
    }

    [Fact]
    public async Task GetCountryById_ShouldReturnCountry()
    {
        var id = Guid.Parse("a1b2c3d4-e5f6-7890-1234-567890abcdef");
        var response = await _client.GetAsync($"/api/countries/{id}");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var item = await response.Content.ReadFromJsonAsync<CountryDto>();
        item.Should().NotBeNull();
        item!.Id.Should().Be(id);
    }

    [Fact]
    public async Task GetStatesByCountry_ShouldReturnList()
    {
        var countryId = Guid.Parse("a1b2c3d4-e5f6-7890-1234-567890abcdef");
        var response = await _client.GetAsync($"/api/countries/{countryId}/states");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<StateDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(s => s.Name == "Madrid");
    }

    [Fact]
    public async Task GetCitiesByState_ShouldReturnList()
    {
        var stateId = Guid.Parse("5d7ac5fb-e49e-442f-88f2-608b8d13d10c"); // State: Madrid
        var response = await _client.GetAsync($"/api/states/{stateId}/cities");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CityDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(c => c.Name == "Madrid");
    }
}
