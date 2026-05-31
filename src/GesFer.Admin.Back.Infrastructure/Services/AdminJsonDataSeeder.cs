using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Domain.Services;
using GesFer.Admin.Back.Domain.ValueObjects;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Hosting;
using System.Text.Json;
using BCrypt.Net;

namespace GesFer.Admin.Back.Infrastructure.Services;

/// <summary>
/// Resultado de la carga de datos de seed para Admin
/// </summary>
public class AdminSeedResult
{
    public bool Loaded { get; set; }
    public List<string> Entities { get; set; } = new();
}

/// <summary>
/// Servicio para cargar datos de seed de Admin desde archivos JSON
/// </summary>
public class AdminJsonDataSeeder
{
    private readonly AdminDbContext _context;
    private readonly ILogger<AdminJsonDataSeeder> _logger;
    private readonly ISensitiveDataSanitizer _sanitizer;
    private readonly IHostEnvironment _hostEnvironment;
    private readonly string _seedsPath;

    public AdminJsonDataSeeder(
        AdminDbContext context,
        ILogger<AdminJsonDataSeeder> logger,
        ISensitiveDataSanitizer sanitizer,
        IHostEnvironment hostEnvironment)
    {
        _context = context;
        _logger = logger;
        _sanitizer = sanitizer;
        _hostEnvironment = hostEnvironment;

        // Obtener la ruta de los archivos de seed
        // Ubicación canónica: src/Admin/Back/Infrastructure/Data/Seeds/
        var basePath = AppContext.BaseDirectory;
        string? foundPath = null;

        // 1. Buscar en Output Directory (Production/Docker)
        var dataSeedsInOutput = Path.Combine(basePath, "Data", "Seeds");
        if (Directory.Exists(dataSeedsInOutput) && HasAnySeedJson(dataSeedsInOutput))
        {
            foundPath = dataSeedsInOutput;
        }
        else
        {
            // 2. Buscar en Source (Development)
            var currentDir = new DirectoryInfo(basePath);
            DirectoryInfo? solutionDir = null;
            var maxDepth = 10;
            var depth = 0;

            while (currentDir != null && solutionDir == null && depth < maxDepth)
            {
                // Buscamos la solución del backend que está en src/
                if (File.Exists(Path.Combine(currentDir.FullName, "GesFer.Admin.Back.sln")))
                {
                    solutionDir = currentDir;
                }
                else
                {
                    currentDir = currentDir.Parent;
                    depth++;
                }
            }

            if (solutionDir != null)
            {
                // Ruta canónica desde la carpeta de la solución (src/)
                // Ajustado a GesFer.Admin.Back.Infrastructure
                var canonicalPath = Path.Combine(solutionDir.FullName, "GesFer.Admin.Back.Infrastructure", "Data", "Seeds");
                if (Directory.Exists(canonicalPath))
                {
                    foundPath = canonicalPath;
                }
            }
        }

        _seedsPath = foundPath ?? Path.Combine(basePath, "Data", "Seeds");

        if (!Directory.Exists(_seedsPath))
        {
            _logger.LogWarning("No se encontró la carpeta de seeds de Admin. Se esperaba en: {Path}", _seedsPath);
        }
        else
        {
            _logger.LogInformation("Carpeta de seeds de Admin encontrada: {Path}", _seedsPath);
        }
    }

    private static bool HasAnySeedJson(string directoryPath)
    {
        return File.Exists(Path.Combine(directoryPath, "admin-users.json"))
            || File.Exists(Path.Combine(directoryPath, "users.json"))
            || File.Exists(Path.Combine(directoryPath, "companies.json"))
            || File.Exists(Path.Combine(directoryPath, "languages.json"))
            || File.Exists(Path.Combine(directoryPath, "postal-codes.json"));
    }

    /// <summary>
    /// Carga todos los seeds de Admin en orden: Languages -> Countries -> States -> Cities -> PostalCodes -> Companies -> AdminUsers.
    /// Responsabilidad única: carga conjunta de datos Admin para BD compartida.
    /// </summary>
    public async Task<AdminSeedResult> SeedAllAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();

        // 1. Languages
        var languagesResult = await SeedLanguagesAsync(cancellationToken);
        if (languagesResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(languagesResult.Entities);
        }

        // 2. Countries
        var countriesResult = await SeedCountriesAsync(cancellationToken);
        if (countriesResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(countriesResult.Entities);
        }

        // 3. States
        var statesResult = await SeedStatesAsync(cancellationToken);
        if (statesResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(statesResult.Entities);
        }

        // 4. Cities
        var citiesResult = await SeedCitiesAsync(cancellationToken);
        if (citiesResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(citiesResult.Entities);
        }

        // 5. PostalCodes
        var postalCodesResult = await SeedPostalCodesAsync(cancellationToken);
        if (postalCodesResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(postalCodesResult.Entities);
        }

        // 6. Companies
        var companiesResult = await SeedCompaniesAsync(cancellationToken);
        if (companiesResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(companiesResult.Entities);
        }

        // 7. Users (multi-tenant)
        var usersResult = await SeedUsersAsync(cancellationToken);
        if (usersResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(usersResult.Entities);
        }

        // 8. AdminUsers (usuarios administrativos)
        var adminUsersResult = await SeedAdminUsersAsync(cancellationToken);
        if (adminUsersResult.Loaded)
        {
            result.Loaded = true;
            result.Entities.AddRange(adminUsersResult.Entities);
        }
        return result;
    }

    /// <summary>
    /// Carga usuarios multi-tenant desde users.json.
    /// </summary>
    public async Task<AdminSeedResult> SeedUsersAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "users.json");
        if (!File.Exists(filePath)) return result;

        _logger.LogInformation("Cargando users desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var users = JsonSerializer.Deserialize<List<UserSeed>>(json, new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        });

        if (users == null || !users.Any()) return result;

        // Cache de companies válidas para evitar FKs/refs inválidas
        var validCompanyIds = new HashSet<Guid>(
            await _context.Companies.IgnoreQueryFilters().AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)
        );

        // Diccionario por (CompanyId, Username) para aplicar idempotencia y reactivación
        var existingUsers = await _context.Users.IgnoreQueryFilters().AsNoTracking().ToListAsync(cancellationToken);
        var existingByKey = existingUsers
            .GroupBy(u => (u.CompanyId, u.Username))
            .ToDictionary(g => g.Key, g => g.First());

        // Hash determinista para admin123 (compatibilidad con test-data)
        const string DeterministicAdmin123Hash = "$2a$11$IRkoFxAcLpHUIwLTqkJaHu6KYx.dgfGY.sFUIsCTY9xHPhL3jcpgW";

        int created = 0;
        int updated = 0;
        int skipped = 0;

        foreach (var u in users)
        {
            if (!Guid.TryParse(u.Id, out var id))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en User '{Username}'. Omitiendo registro.", u.Id, u.Username);
                skipped++;
                continue;
            }
            if (!Guid.TryParse(u.CompanyId, out var companyId) || !validCompanyIds.Contains(companyId))
            {
                _logger.LogWarning("AdminJsonDataSeeder: CompanyId '{CompanyId}' no es válido o no existe para User '{Username}'. Omitiendo registro.",
                    u.CompanyId, u.Username);
                skipped++;
                continue;
            }

            Guid? postalCodeId = TryParseNullableGuid(u.PostalCodeId, "PostalCodeId", u.Username);
            Guid? cityId = TryParseNullableGuid(u.CityId, "CityId", u.Username);
            Guid? stateId = TryParseNullableGuid(u.StateId, "StateId", u.Username);
            Guid? countryId = TryParseNullableGuid(u.CountryId, "CountryId", u.Username);
            Guid? languageId = TryParseNullableGuid(u.LanguageId, "LanguageId", u.Username);

            Email? email = null;
            if (!string.IsNullOrWhiteSpace(u.Email))
            {
                if (!Email.TryCreate(u.Email, out var parsedEmail))
                {
                    _logger.LogWarning("AdminJsonDataSeeder: Email inválido '{Email}' en User '{Username}'. Omitiendo registro.", u.Email, u.Username);
                    skipped++;
                    continue;
                }
                email = parsedEmail;
            }

            var key = (companyId, u.Username);
            existingByKey.TryGetValue(key, out var existing);

            var passwordHash = ResolvePasswordHash(u.Password, DeterministicAdmin123Hash);

            if (existing == null)
            {
                _context.Users.Add(new Domain.Entities.User
                {
                    Id = id,
                    CompanyId = companyId,
                    Username = u.Username,
                    PasswordHash = passwordHash,
                    FirstName = u.FirstName ?? string.Empty,
                    LastName = u.LastName ?? string.Empty,
                    Email = email,
                    Phone = string.IsNullOrWhiteSpace(u.Phone) ? null : u.Phone,
                    Address = string.IsNullOrWhiteSpace(u.Address) ? null : u.Address,
                    PostalCodeId = postalCodeId,
                    CityId = cityId,
                    StateId = stateId,
                    CountryId = countryId,
                    LanguageId = languageId,
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                });
                created++;
            }
            else
            {
                bool modified = false;

                // Reactivación soft delete si aplica
                if (existing.DeletedAt != null)
                {
                    existing.DeletedAt = null;
                    existing.IsActive = true;
                    modified = true;
                }

                // Actualizar campos básicos (mantener idempotencia)
                if (existing.FirstName != (u.FirstName ?? string.Empty)) { existing.FirstName = u.FirstName ?? string.Empty; modified = true; }
                if (existing.LastName != (u.LastName ?? string.Empty)) { existing.LastName = u.LastName ?? string.Empty; modified = true; }
                if (existing.Phone != (string.IsNullOrWhiteSpace(u.Phone) ? null : u.Phone)) { existing.Phone = string.IsNullOrWhiteSpace(u.Phone) ? null : u.Phone; modified = true; }
                if (existing.Address != (string.IsNullOrWhiteSpace(u.Address) ? null : u.Address)) { existing.Address = string.IsNullOrWhiteSpace(u.Address) ? null : u.Address; modified = true; }
                if (existing.PostalCodeId != postalCodeId) { existing.PostalCodeId = postalCodeId; modified = true; }
                if (existing.CityId != cityId) { existing.CityId = cityId; modified = true; }
                if (existing.StateId != stateId) { existing.StateId = stateId; modified = true; }
                if (existing.CountryId != countryId) { existing.CountryId = countryId; modified = true; }
                if (existing.LanguageId != languageId) { existing.LanguageId = languageId; modified = true; }
                if (existing.Email != email) { existing.Email = email; modified = true; }

                // Actualizar password si el seed trae password y no coincide
                if (!string.IsNullOrWhiteSpace(u.Password) && !BCrypt.Net.BCrypt.Verify(u.Password, existing.PasswordHash))
                {
                    existing.PasswordHash = passwordHash;
                    modified = true;
                }

                if (modified) updated++;
            }
        }

        if (created > 0 || updated > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            if (created > 0) result.Entities.Add($"{created} Users created");
            if (updated > 0) result.Entities.Add($"{updated} Users updated");
        }
        else
        {
            result.Loaded = true;
            result.Entities.Add("No new Users");
        }

        if (skipped > 0)
            _logger.LogWarning("[SEED ADMIN] Users: {Skipped} ignorado(s)", skipped);

        return result;
    }

    private Guid? TryParseNullableGuid(string? raw, string field, string username)
    {
        if (string.IsNullOrWhiteSpace(raw)) return null;
        if (Guid.TryParse(raw, out var value)) return value;
        _logger.LogWarning("AdminJsonDataSeeder: {Field} '{Raw}' no es válido en User '{Username}'. Asignando null.", field, raw, username);
        return null;
    }

    private static string ResolvePasswordHash(string? rawPassword, string deterministicAdmin123Hash)
    {
        if (string.IsNullOrWhiteSpace(rawPassword))
        {
            // Si falta password, generamos una estándar (DEV/TEST), manteniendo la política existente
            rawPassword = "admin123";
        }

        if (rawPassword == "admin123")
            return deterministicAdmin123Hash;

        return BCrypt.Net.BCrypt.HashPassword(rawPassword);
    }

    public async Task<AdminSeedResult> SeedLanguagesAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "languages.json");
        if (!File.Exists(filePath)) return result;

        _logger.LogInformation("Cargando languages desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var languages = JsonSerializer.Deserialize<List<LanguageSeed>>(json, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });

        if (languages == null || !languages.Any()) return result;

        var existingIds = new HashSet<Guid>(
            await _context.Languages.IgnoreQueryFilters().AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)
        );

        int count = 0;
        foreach (var item in languages)
        {
            if (!Guid.TryParse(item.Id, out var id))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en Language '{Name}'. Omitiendo registro.", item.Id, item.Name);
                continue;
            }

            if (!existingIds.Contains(id))
            {
                _context.Languages.Add(new Language
                {
                    Id = id,
                    Name = item.Name,
                    Code = item.Code,
                    Description = item.Description,
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                });
                count++;
            }
        }
        if (count > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{count} Languages created");
        }
        return result;
    }

    public async Task<AdminSeedResult> SeedCountriesAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "countries.json");
        if (!File.Exists(filePath)) return result;

        _logger.LogInformation("Cargando countries desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var countries = JsonSerializer.Deserialize<List<CountrySeed>>(json, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });

        if (countries == null || !countries.Any()) return result;

        var existingIds = new HashSet<Guid>(
            await _context.Countries.IgnoreQueryFilters().AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)
        );

        int count = 0;
        foreach (var item in countries)
        {
            if (!Guid.TryParse(item.Id, out var id))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en Country '{Name}'. Omitiendo registro.", item.Id, item.Name);
                continue;
            }
            if (!Guid.TryParse(item.LanguageId, out var languageId))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El LanguageId '{LanguageId}' no es válido en Country '{Name}'. Omitiendo registro.", item.LanguageId, item.Name);
                continue;
            }

            if (!existingIds.Contains(id))
            {
                _context.Countries.Add(new Country
                {
                    Id = id,
                    Name = item.Name,
                    Code = item.Code,
                    LanguageId = languageId,
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                });
                count++;
            }
        }
        if (count > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{count} Countries created");
        }
        return result;
    }

    public async Task<AdminSeedResult> SeedStatesAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "states.json");
        if (!File.Exists(filePath)) return result;

        _logger.LogInformation("Cargando states desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var states = JsonSerializer.Deserialize<List<StateSeed>>(json, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });

        if (states == null || !states.Any()) return result;

        var existingIds = new HashSet<Guid>(
            await _context.States.IgnoreQueryFilters().AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)
        );

        int count = 0;
        foreach (var item in states)
        {
            if (!Guid.TryParse(item.Id, out var id))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en State '{Name}'. Omitiendo registro.", item.Id, item.Name);
                continue;
            }
            if (!Guid.TryParse(item.CountryId, out var countryId))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El CountryId '{CountryId}' no es válido en State '{Name}'. Omitiendo registro.", item.CountryId, item.Name);
                continue;
            }

            if (!existingIds.Contains(id))
            {
                _context.States.Add(new State
                {
                    Id = id,
                    CountryId = countryId,
                    Name = item.Name,
                    Code = item.Code,
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                });
                count++;
            }
        }
        if (count > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{count} States created");
        }
        return result;
    }

    public async Task<AdminSeedResult> SeedCitiesAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "cities.json");
        if (!File.Exists(filePath)) return result;

        _logger.LogInformation("Cargando cities desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var cities = JsonSerializer.Deserialize<List<CitySeed>>(json, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });

        if (cities == null || !cities.Any()) return result;

        var existingIds = new HashSet<Guid>(
            await _context.Cities.IgnoreQueryFilters().AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)
        );

        int count = 0;
        foreach (var item in cities)
        {
            if (!Guid.TryParse(item.Id, out var id))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en City '{Name}'. Omitiendo registro.", item.Id, item.Name);
                continue;
            }
            if (!Guid.TryParse(item.StateId, out var stateId))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El StateId '{StateId}' no es válido en City '{Name}'. Omitiendo registro.", item.StateId, item.Name);
                continue;
            }

            if (!existingIds.Contains(id))
            {
                _context.Cities.Add(new City
                {
                    Id = id,
                    StateId = stateId,
                    Name = item.Name,
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                });
                count++;
            }
        }
        if (count > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{count} Cities created");
        }
        return result;
    }

    /// <summary>
    /// Carga códigos postales desde postal-codes.json (maestro geo).
    /// </summary>
    public async Task<AdminSeedResult> SeedPostalCodesAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "postal-codes.json");
        if (!File.Exists(filePath)) return result;

        _logger.LogInformation("Cargando códigos postales desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var postalCodes = JsonSerializer.Deserialize<List<PostalCodeSeed>>(json, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });

        if (postalCodes == null || !postalCodes.Any()) return result;

        var existingIds = new HashSet<Guid>(
            await _context.PostalCodes.IgnoreQueryFilters().AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)
        );

        int count = 0;
        foreach (var item in postalCodes)
        {
            if (!Guid.TryParse(item.Id, out var id))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en PostalCode '{Code}'. Omitiendo registro.", item.Id, item.Code);
                continue;
            }
            if (!Guid.TryParse(item.CityId, out var cityId))
            {
                _logger.LogWarning("AdminJsonDataSeeder: El CityId '{CityId}' no es válido en PostalCode '{Code}'. Omitiendo registro.", item.CityId, item.Code);
                continue;
            }

            if (!existingIds.Contains(id))
            {
                _context.PostalCodes.Add(new PostalCode
                {
                    Id = id,
                    CityId = cityId,
                    Code = item.Code,
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                });
                count++;
            }
        }
        if (count > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{count} PostalCodes created");
        }
        return result;
    }

    /// <summary>
    /// Carga usuarios administrativos desde admin-users.json
    /// </summary>
    public async Task<AdminSeedResult> SeedAdminUsersAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "admin-users.json");
        if (!File.Exists(filePath))
        {
            _logger.LogWarning("Archivo admin-users.json no encontrado en {Path}", filePath);
            return result;
        }

        _logger.LogInformation("Cargando usuarios admin desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var users = JsonSerializer.Deserialize<List<AdminUserSeed>>(json, new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        });

        if (users == null || !users.Any())
        {
             _logger.LogWarning("No se encontraron usuarios en admin-users.json");
            return result;
        }

        var allUsers = await _context.AdminUsers.IgnoreQueryFilters().AsNoTracking().ToListAsync(cancellationToken);
        var existingUsersDict = allUsers
            .GroupBy(u => u.Username)
            .ToDictionary(g => g.Key, g => g.First());

        int count = 0;
        foreach (var userData in users)
        {
            if (userData.Username == null) continue;
            existingUsersDict.TryGetValue(userData.Username, out var existing);

            string passwordHash;
            if (string.IsNullOrEmpty(userData.Password))
            {
                string rawPassword;
                if (_hostEnvironment.IsDevelopment() || _hostEnvironment.IsEnvironment("Testing"))
                {
                    rawPassword = "admin123";
                    _logger.LogWarning("[SEED ADMIN] ⚠️ DEV/TEST MODE: Setting fixed password '{Password}' for Admin '{Username}'", rawPassword, userData.Username);
                }
                else
                {
                    rawPassword = _sanitizer.GenerateRandomPassword();
                    _logger.LogWarning("[SEED ADMIN] 🔐 Generated RANDOM password for Admin '{Username}': {Password}", userData.Username, rawPassword);
                }
                passwordHash = BCrypt.Net.BCrypt.HashPassword(rawPassword);
            }
            else
            {
                passwordHash = BCrypt.Net.BCrypt.HashPassword(userData.Password);
            }

            if (existing == null)
            {
                Guid id;
                if (!Guid.TryParse(userData.Id, out id))
                {
                    id = Guid.NewGuid();
                }

                var user = new AdminUser
                {
                    Id = id,
                    Username = userData.Username,
                    PasswordHash = passwordHash,
                    FirstName = userData.FirstName,
                    LastName = userData.LastName,
                    Email = userData.Email,
                    Role = userData.Role ?? "Admin",
                    CreatedAt = DateTime.UtcNow,
                    IsActive = true
                };
                _context.AdminUsers.Add(user);
                count++;
                _logger.LogInformation("[SEED ADMIN] Creado usuario admin: {Username}", userData.Username);
            }
            else
            {
                bool modified = false;
                if (existing.DeletedAt != null)
                {
                    existing.DeletedAt = null;
                    existing.IsActive = true;
                    modified = true;
                    _logger.LogInformation("[SEED ADMIN] Reactivado usuario admin: {Username}", userData.Username);
                }

                // Actualizar contraseña si es un seed (para asegurar que coincida)
                // Esto podría ser debatible en prod, pero para seed/reset es útil.
                // Verificamos si la contraseña ha cambiado
                // Si la password en JSON es vacía (random), NO la actualizamos si ya existe, para no sobrescribir la del usuario.
                if (!string.IsNullOrEmpty(userData.Password) && !BCrypt.Net.BCrypt.Verify(userData.Password, existing.PasswordHash))
                {
                    existing.PasswordHash = passwordHash;
                    modified = true;
                    _logger.LogInformation("[SEED ADMIN] Actualizada contraseña usuario admin: {Username}", userData.Username);
                }

                if (modified) count++;
            }
        }

        if (count > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{count} Admin User(s) created/updated");
        }
        else
        {
             result.Loaded = true; // Loaded checked, nothing new
             result.Entities.Add("No new Admin Users");
        }

        return result;
    }

    /// <summary>
    /// Carga empresas (Companies) desde companies.json usando AdminDbContext.
    /// Admin es SSOT para Company; en entornos con BD compartida, ejecutar este seed antes que el de Product.
    /// </summary>
    public async Task<AdminSeedResult> SeedCompaniesAsync(CancellationToken cancellationToken = default)
    {
        var result = new AdminSeedResult();
        var filePath = Path.Combine(_seedsPath, "companies.json");
        if (!File.Exists(filePath))
        {
            _logger.LogWarning("Archivo companies.json no encontrado en {Path}", filePath);
            return result;
        }

        _logger.LogInformation("Cargando companies desde {Path}", filePath);
        var json = await File.ReadAllTextAsync(filePath);
        var companies = JsonSerializer.Deserialize<List<CompanySeed>>(json, new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true
        });

        if (companies == null || !companies.Any())
        {
            _logger.LogWarning("No se encontraron companies en companies.json");
            return result;
        }

        var allCompanies = await _context.Companies.IgnoreQueryFilters().AsNoTracking().ToListAsync(cancellationToken);
        var existingCompaniesDict = allCompanies
            .GroupBy(c => c.Id)
            .ToDictionary(g => g.Key, g => g.First());

        int processedCount = 0;
        int skippedCount = 0;

        foreach (var companyData in companies)
        {
            try
            {
                if (!Guid.TryParse(companyData.Id, out var id))
                {
                    _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido en Company '{Name}'. Omitiendo registro.", companyData.Id, companyData.Name);
                    skippedCount++;
                    continue;
                }

                existingCompaniesDict.TryGetValue(id, out var existing);

                if (existing == null)
                {
                    TaxId? taxId = null;
                    if (!string.IsNullOrWhiteSpace(companyData.TaxId))
                    {
                        if (!TaxId.TryCreate(companyData.TaxId, out var parsedTaxId))
                        {
                            _logger.LogWarning("[SEED ADMIN] TaxId inválido en Company '{Name}' (Id: {Id}). Registro ignorado.",
                                companyData.Name, companyData.Id);
                            skippedCount++;
                            continue;
                        }
                        taxId = parsedTaxId;
                    }

                    Email? email = null;
                    if (!string.IsNullOrWhiteSpace(companyData.Email))
                    {
                        if (!Email.TryCreate(companyData.Email, out var parsedEmail))
                        {
                            _logger.LogWarning("[SEED ADMIN] Email inválido en Company '{Name}' (Id: {Id}). Registro ignorado.",
                                companyData.Name, companyData.Id);
                            skippedCount++;
                            continue;
                        }
                        email = parsedEmail;
                    }

                    Guid? languageId = null;
                    if (!string.IsNullOrWhiteSpace(companyData.LanguageId))
                    {
                        if (Guid.TryParse(companyData.LanguageId, out var parsedLanguageId))
                        {
                            languageId = parsedLanguageId;
                        }
                        else
                        {
                            _logger.LogWarning("[SEED ADMIN] LanguageId inválido en Company '{Name}' (Id: {Id}). Asignando null.",
                                companyData.Name, companyData.Id);
                        }
                    }

                    var company = new Company
                    {
                        Id = id,
                        Name = companyData.Name,
                        TaxId = taxId,
                        Address = companyData.Address,
                        Phone = string.IsNullOrWhiteSpace(companyData.Phone) ? null : companyData.Phone,
                        Email = email,
                        LanguageId = languageId,
                        CreatedAt = DateTime.UtcNow,
                        IsActive = true
                    };
                    _context.Companies.Add(company);
                    processedCount++;
                    _logger.LogInformation("[SEED ADMIN] Creada company: '{Name}' (Id: {Id})", companyData.Name, companyData.Id);
                }
                else if (existing.DeletedAt != null)
                {
                    existing.DeletedAt = null;
                    existing.IsActive = true;
                    processedCount++;
                    _logger.LogInformation("[SEED ADMIN] Reactivada company: '{Name}' (Id: {Id})", companyData.Name, companyData.Id);
                }
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "[SEED ADMIN] Error al procesar Company '{Name}' (Id: {Id})", companyData.Name, companyData.Id);
                skippedCount++;
            }
        }

        if (processedCount > 0)
        {
            await _context.SaveChangesAsync(cancellationToken);
            result.Loaded = true;
            result.Entities.Add($"{processedCount} Company(ies)");
        }
        if (skippedCount > 0)
            _logger.LogWarning("[SEED ADMIN] Companies: {Skipped} ignorado(s)", skippedCount);

        return result;
    }

    private class CompanySeed
    {
        public string Id { get; set; } = string.Empty;
        public string Name { get; set; } = string.Empty;
        public string? TaxId { get; set; }
        public string Address { get; set; } = string.Empty;
        public string? Phone { get; set; }
        public string? Email { get; set; }
        public string LanguageId { get; set; } = string.Empty;
    }

    private class UserSeed
    {
        public string Id { get; set; } = string.Empty;
        public string CompanyId { get; set; } = string.Empty;
        public string Username { get; set; } = string.Empty;
        public string Password { get; set; } = string.Empty;
        public string? FirstName { get; set; }
        public string? LastName { get; set; }
        public string? Email { get; set; }
        public string? Phone { get; set; }
        public string? Address { get; set; }
        public string? PostalCodeId { get; set; }
        public string? CityId { get; set; }
        public string? StateId { get; set; }
        public string? CountryId { get; set; }
        public string? LanguageId { get; set; }
    }

    private class AdminUserSeed
    {
        public string Id { get; set; } = string.Empty;
        public string Username { get; set; } = string.Empty;
        public string Password { get; set; } = string.Empty;
        public string FirstName { get; set; } = string.Empty;
        public string LastName { get; set; } = string.Empty;
        public string? Email { get; set; }
        public string? Role { get; set; }
    }

    private class LanguageSeed
    {
        public string Id { get; set; } = string.Empty;
        public string Name { get; set; } = string.Empty;
        public string Code { get; set; } = string.Empty;
        public string? Description { get; set; }
    }

    private class CountrySeed
    {
        public string Id { get; set; } = string.Empty;
        public string Name { get; set; } = string.Empty;
        public string Code { get; set; } = string.Empty;
        public string LanguageId { get; set; } = string.Empty;
    }

    private class StateSeed
    {
        public string Id { get; set; } = string.Empty;
        public string CountryId { get; set; } = string.Empty;
        public string Name { get; set; } = string.Empty;
        public string? Code { get; set; }
    }

    private class CitySeed
    {
        public string Id { get; set; } = string.Empty;
        public string StateId { get; set; } = string.Empty;
        public string Name { get; set; } = string.Empty;
    }

    private class PostalCodeSeed
    {
        public string Id { get; set; } = string.Empty;
        public string CityId { get; set; } = string.Empty;
        public string Code { get; set; } = string.Empty;
    }
}
