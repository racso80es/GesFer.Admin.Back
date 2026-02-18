# GesFer.Admin.Back

Backend (API) del módulo de administración GesFer. Este repositorio contiene **solo** la API y sus capas de aplicación, infraestructura y dominio; el proyecto se mantiene aislado respecto a otras piezas del ecosistema.

## Contexto

- **Stack:** .NET 8, ASP.NET Core Web API, JWT, Entity Framework Core, Serilog, Swagger.
- **Estructura:** Api → Application → Infrastructure → Domain; tests y scripts en `src/`.

## Requisitos

- .NET 8 SDK
- Windows 11 + PowerShell 7+ (según convención del proyecto; ver `AGENTS.md`)

## Ejecución

Desde la raíz del repositorio, en PowerShell:

```powershell
dotnet run --project src/Api/GesFer.Admin.Api.csproj
```

La API expone Swagger en la ruta habitual (según configuración del proyecto).

## Documentación de objetivos

Los objetivos y el alcance del proyecto están documentados en **[Objetivos.md](./Objetivos.md)**.

## Protocolo del proyecto

Para convenciones, leyes universales y sistema multi-agente, ver **[AGENTS.md](./AGENTS.md)**.
