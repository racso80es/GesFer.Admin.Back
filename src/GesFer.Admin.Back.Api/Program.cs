using GesFer.Admin.Back.Api;
using GesFer.Admin.Back.Infrastructure;
using GesFer.Admin.Back.Infrastructure.Logging;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.Extensions.Logging;
using Microsoft.AspNetCore.HttpsPolicy;
using Microsoft.EntityFrameworkCore;
using Microsoft.IdentityModel.Tokens;
using System.Text;

var builder = WebApplication.CreateBuilder(args);

// Cola de logs: instancia explícita para el provider y para DI
var logQueue = new LogQueue();
builder.Services.AddSingleton<ILogQueue>(logQueue);

// Logging sin Serilog: provider custom que escribe a ILogQueue
builder.Logging.AddProvider(new LogQueueLoggerProvider(logQueue));
var isDevelopment = builder.Environment.IsDevelopment();
builder.Logging.SetMinimumLevel(isDevelopment ? LogLevel.Debug : LogLevel.Information);
builder.Logging.AddFilter("Microsoft", LogLevel.Warning);
builder.Logging.AddFilter("Microsoft.AspNetCore", LogLevel.Warning);

// Configurar servicios
builder.Services.AddControllers();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
    {
        c.SwaggerDoc("v1", new Microsoft.OpenApi.Models.OpenApiInfo
        {
            Title = "GesFer Admin API",
            Version = "v1",
            Description = "API RESTful para gestión administrativa del sistema GesFer"
        });
        
        // Configurar seguridad JWT en Swagger (Http + Bearer para que Swagger UI envíe "Authorization: Bearer {token}")
        c.AddSecurityDefinition("Bearer", new Microsoft.OpenApi.Models.OpenApiSecurityScheme
        {
            Description = "JWT Authorization header usando el esquema Bearer. Introduce solo el token (sin 'Bearer ').",
            Name = "Authorization",
            In = Microsoft.OpenApi.Models.ParameterLocation.Header,
            Type = Microsoft.OpenApi.Models.SecuritySchemeType.Http,
            Scheme = "bearer",
            BearerFormat = "JWT"
        });

        c.AddSecurityRequirement(new Microsoft.OpenApi.Models.OpenApiSecurityRequirement
        {
            {
                new Microsoft.OpenApi.Models.OpenApiSecurityScheme
                {
                    Reference = new Microsoft.OpenApi.Models.OpenApiReference
                    {
                        Type = Microsoft.OpenApi.Models.ReferenceType.SecurityScheme,
                        Id = "Bearer"
                    }
                },
                Array.Empty<string>()
            }
        });
    c.UseInlineDefinitionsForEnums();
    c.ResolveConflictingActions(apiDescriptions => apiDescriptions.First());
});

// Configurar CORS
var allowedOrigins = builder.Configuration.GetSection("Cors:AllowedOrigins").Get<string[]>() ?? Array.Empty<string>();

builder.Services.AddCors(options =>
{
    options.AddPolicy("AllowAll", policy =>
    {
        policy.WithOrigins(allowedOrigins)
              .AllowAnyMethod()
              .AllowAnyHeader();
    });
});

// Seguridad: HTTPS en todos los entornos. Redirección HTTP → HTTPS.
if (isDevelopment)
    builder.Services.Configure<HttpsRedirectionOptions>(options => { options.HttpsPort = 5011; });

// Configurar opciones
builder.Services.Configure<GesFer.Admin.Back.Api.Configuration.SystemAuthOptions>(options =>
{
    options.SharedSecret = builder.Configuration["SharedSecret"] ?? string.Empty;
});

// Configurar inyección de dependencias
builder.Services.AddApplicationServices(builder.Configuration, builder.Environment);
builder.Services.AddInfrastructureServices(builder.Configuration, builder.Environment.IsDevelopment());

// Healthchecks
builder.Services.AddHealthChecks();

// Configurar autenticación JWT (misma configuración que Product)
var jwtSecretKey = builder.Configuration["JwtSettings:SecretKey"]
    ?? throw new InvalidOperationException("JwtSettings:SecretKey no está configurado");

if (jwtSecretKey.Length < 32)
{
    throw new InvalidOperationException(
        $"JwtSettings:SecretKey debe tener al menos 32 caracteres (256 bits) para cumplir con el algoritmo SHA-256 (HS256). " +
        $"Longitud actual: {jwtSecretKey.Length} caracteres.");
}

var jwtIssuer = builder.Configuration["JwtSettings:Issuer"]
    ?? throw new InvalidOperationException("JwtSettings:Issuer no está configurado");
var jwtAudience = builder.Configuration["JwtSettings:Audience"]
    ?? throw new InvalidOperationException("JwtSettings:Audience no está configurado");

builder.Services.AddAuthentication(options =>
{
    options.DefaultAuthenticateScheme = JwtBearerDefaults.AuthenticationScheme;
    options.DefaultChallengeScheme = JwtBearerDefaults.AuthenticationScheme;
})
.AddJwtBearer(options =>
{
    options.TokenValidationParameters = new TokenValidationParameters
    {
        ValidateIssuer = true,
        ValidateAudience = true,
        ValidateLifetime = true,
        ValidateIssuerSigningKey = true,
        ValidIssuer = jwtIssuer,
        ValidAudience = jwtAudience,
        IssuerSigningKey = new SymmetricSecurityKey(Encoding.UTF8.GetBytes(jwtSecretKey)),
        ClockSkew = TimeSpan.Zero,
        RoleClaimType = System.Security.Claims.ClaimTypes.Role
    };
});

builder.Services.AddAuthorization(options =>
{
    // Política de autorización que exige el claim role: Admin
    options.AddPolicy("AdminOnly", policy =>
    {
        policy.RequireAuthenticatedUser();
        policy.RequireRole("Admin");
    });
});

var app = builder.Build();

// Modo solo seeds: ejecutar migraciones + seeds y salir (para scripts/tools/Invoke-MySqlSeeds.ps1)
if (Environment.GetEnvironmentVariable("RUN_SEEDS_ONLY") == "1")
{
    await app.Services.RunMigrationsAndSeedsThenExitAsync();
}

// Inicializar base de datos (solo migraciones; seeds vía Invoke-MySqlSeeds con RUN_SEEDS_ONLY=1)
await app.Services.RunMigrationsOnlyAsync();

// Configurar el pipeline HTTP
app.UseMiddleware<GesFer.Admin.Back.Api.Middleware.GlobalExceptionHandlingMiddleware>();

if (app.Environment.IsDevelopment() || app.Environment.IsEnvironment("Testing"))
{
    if (app.Environment.IsDevelopment())
        app.UseDeveloperExceptionPage();
    app.UseSwagger();
    app.UseSwaggerUI(c =>
    {
        c.SwaggerEndpoint("/swagger/v1/swagger.json", "GesFer Admin API v1");
        c.RoutePrefix = string.Empty;
    });
}

// Redirección HTTP → HTTPS: omitir en Development para que Swagger funcione solo con HTTP (5010).
// Si se redirige a 5011 y el perfil solo enlaza 5010, el fetch de swagger.json falla con 500/Fetch error.
if (!app.Environment.IsDevelopment() && !app.Environment.IsEnvironment("Testing"))
{
    app.UseHttpsRedirection();
}
app.UseRouting();
app.UseCors("AllowAll");


// Autenticación y autorización deben ir en este orden
app.UseAuthentication();
app.UseAuthorization();

app.MapHealthChecks("/health");
app.MapControllers();

app.Run();

public partial class Program { }
