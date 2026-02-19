FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /app
COPY . .
RUN dotnet restore src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj
RUN dotnet publish src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj -c Release -o /app/publish

FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS final
WORKDIR /app
COPY --from=build /app/publish .
EXPOSE 5000
EXPOSE 5001
ENTRYPOINT ["dotnet", "GesFer.Admin.Back.Api.dll"]
