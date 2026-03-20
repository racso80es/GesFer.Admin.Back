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
        var response = await _client.GetAsync("/api/geolocation/countries");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CountryGeoReadDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(c => c.Name == "España");
    }

    [Fact]
    public async Task GetCountryById_ShouldReturnCountry()
    {
        var id = Guid.Parse("a1b2c3d4-e5f6-7890-1234-567890abcdef");
        var response = await _client.GetAsync($"/api/geolocation/countries/{id}");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var item = await response.Content.ReadFromJsonAsync<CountryGeoReadDto>();
        item.Should().NotBeNull();
        item!.Id.Should().Be(id);
    }

    [Fact]
    public async Task GetStatesByCountry_ShouldReturnList()
    {
        var countryId = Guid.Parse("a1b2c3d4-e5f6-7890-1234-567890abcdef");
        var response = await _client.GetAsync($"/api/geolocation/countries/{countryId}/states");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<StateGeoReadDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(s => s.Name == "Madrid");
    }

    [Fact]
    public async Task GetCitiesByState_ShouldReturnList()
    {
        var stateId = Guid.Parse("5d7ac5fb-e49e-442f-88f2-608b8d13d10c"); // State: Madrid
        var response = await _client.GetAsync($"/api/geolocation/states/{stateId}/cities");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<CityGeoReadDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(c => c.Name == "Madrid");
    }

    /// <summary>Ciudad Madrid (seed) y código postal 28023 (seed).</summary>
    [Fact]
    public async Task GetPostalCodesByCity_ShouldReturnList()
    {
        var cityId = Guid.Parse("82bbd055-db08-4371-b764-5bfe6a664239");
        var response = await _client.GetAsync($"/api/geolocation/cities/{cityId}/postal-codes");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
        var list = await response.Content.ReadFromJsonAsync<List<PostalCodeGeoReadDto>>();
        list.Should().NotBeNull();
        list.Should().Contain(p => p.Code == "28023");
    }
}
