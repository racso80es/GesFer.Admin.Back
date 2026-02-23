using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace GesFer.Admin.Back.Infrastructure.Data.Migrations
{
    /// <summary>
    /// Crea tablas de geo y Companies para Admin (idempotente).
    /// Orden: Language → Country → State → City → PostalCode → Companies.
    /// Según spec estabilidad-bd-inicializacion; clarify: CREATE TABLE IF NOT EXISTS.
    /// </summary>
    public partial class AddAdminCoreAndGeoTables : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            // Language (sin FKs)
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `Language` (
    `Id` char(36) NOT NULL COLLATE ascii_general_ci,
    `Code` longtext NOT NULL,
    `Name` longtext NOT NULL,
    `Description` longtext NULL,
    `CreatedAt` datetime(6) NOT NULL,
    `UpdatedAt` datetime(6) NULL,
    `DeletedAt` datetime(6) NULL,
    `IsActive` tinyint(1) NOT NULL,
    PRIMARY KEY (`Id`)
) CHARACTER SET utf8mb4;
");

            // Country (FK LanguageId)
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `Country` (
    `Id` char(36) NOT NULL COLLATE ascii_general_ci,
    `Code` longtext NOT NULL,
    `Name` longtext NOT NULL,
    `LanguageId` char(36) NOT NULL COLLATE ascii_general_ci,
    `CreatedAt` datetime(6) NOT NULL,
    `UpdatedAt` datetime(6) NULL,
    `DeletedAt` datetime(6) NULL,
    `IsActive` tinyint(1) NOT NULL,
    PRIMARY KEY (`Id`),
    INDEX `IX_Country_LanguageId` (`LanguageId`),
    CONSTRAINT `FK_Country_Language_LanguageId` FOREIGN KEY (`LanguageId`) REFERENCES `Language` (`Id`) ON DELETE CASCADE
) CHARACTER SET utf8mb4;
");

            // State (FK CountryId)
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `State` (
    `Id` char(36) NOT NULL COLLATE ascii_general_ci,
    `Code` longtext NULL,
    `Name` longtext NOT NULL,
    `CountryId` char(36) NOT NULL COLLATE ascii_general_ci,
    `CreatedAt` datetime(6) NOT NULL,
    `UpdatedAt` datetime(6) NULL,
    `DeletedAt` datetime(6) NULL,
    `IsActive` tinyint(1) NOT NULL,
    PRIMARY KEY (`Id`),
    INDEX `IX_State_CountryId` (`CountryId`),
    CONSTRAINT `FK_State_Country_CountryId` FOREIGN KEY (`CountryId`) REFERENCES `Country` (`Id`) ON DELETE CASCADE
) CHARACTER SET utf8mb4;
");

            // City (FK StateId)
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `City` (
    `Id` char(36) NOT NULL COLLATE ascii_general_ci,
    `Name` longtext NOT NULL,
    `StateId` char(36) NOT NULL COLLATE ascii_general_ci,
    `CreatedAt` datetime(6) NOT NULL,
    `UpdatedAt` datetime(6) NULL,
    `DeletedAt` datetime(6) NULL,
    `IsActive` tinyint(1) NOT NULL,
    PRIMARY KEY (`Id`),
    INDEX `IX_City_StateId` (`StateId`),
    CONSTRAINT `FK_City_State_StateId` FOREIGN KEY (`StateId`) REFERENCES `State` (`Id`) ON DELETE CASCADE
) CHARACTER SET utf8mb4;
");

            // PostalCode (FK CityId)
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `PostalCode` (
    `Id` char(36) NOT NULL COLLATE ascii_general_ci,
    `Code` longtext NOT NULL,
    `CityId` char(36) NOT NULL COLLATE ascii_general_ci,
    `CreatedAt` datetime(6) NOT NULL,
    `UpdatedAt` datetime(6) NULL,
    `DeletedAt` datetime(6) NULL,
    `IsActive` tinyint(1) NOT NULL,
    PRIMARY KEY (`Id`),
    INDEX `IX_PostalCode_CityId` (`CityId`),
    CONSTRAINT `FK_PostalCode_City_CityId` FOREIGN KEY (`CityId`) REFERENCES `City` (`Id`) ON DELETE CASCADE
) CHARACTER SET utf8mb4;
");

            // Companies (FKs opcionales a City, Country, State, Language, PostalCode)
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `Companies` (
    `Id` char(36) NOT NULL COLLATE ascii_general_ci,
    `Name` varchar(200) NOT NULL,
    `Address` varchar(500) NOT NULL,
    `Email` varchar(200) NULL,
    `Phone` varchar(50) NULL,
    `TaxId` varchar(50) NULL,
    `CityId` char(36) NULL COLLATE ascii_general_ci,
    `CountryId` char(36) NULL COLLATE ascii_general_ci,
    `StateId` char(36) NULL COLLATE ascii_general_ci,
    `LanguageId` char(36) NULL COLLATE ascii_general_ci,
    `PostalCodeId` char(36) NULL COLLATE ascii_general_ci,
    `CreatedAt` datetime(6) NOT NULL,
    `UpdatedAt` datetime(6) NULL,
    `DeletedAt` datetime(6) NULL,
    `IsActive` tinyint(1) NOT NULL,
    PRIMARY KEY (`Id`),
    INDEX `IX_Companies_Name` (`Name`),
    INDEX `IX_Companies_CityId` (`CityId`),
    INDEX `IX_Companies_CountryId` (`CountryId`),
    INDEX `IX_Companies_StateId` (`StateId`),
    INDEX `IX_Companies_LanguageId` (`LanguageId`),
    INDEX `IX_Companies_PostalCodeId` (`PostalCodeId`),
    CONSTRAINT `FK_Companies_City_CityId` FOREIGN KEY (`CityId`) REFERENCES `City` (`Id`) ON DELETE RESTRICT,
    CONSTRAINT `FK_Companies_Country_CountryId` FOREIGN KEY (`CountryId`) REFERENCES `Country` (`Id`) ON DELETE RESTRICT,
    CONSTRAINT `FK_Companies_State_StateId` FOREIGN KEY (`StateId`) REFERENCES `State` (`Id`) ON DELETE RESTRICT,
    CONSTRAINT `FK_Companies_Language_LanguageId` FOREIGN KEY (`LanguageId`) REFERENCES `Language` (`Id`) ON DELETE RESTRICT,
    CONSTRAINT `FK_Companies_PostalCode_PostalCodeId` FOREIGN KEY (`PostalCodeId`) REFERENCES `PostalCode` (`Id`) ON DELETE RESTRICT
) CHARACTER SET utf8mb4;
");
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            // Orden inverso: eliminar FKs primero implícito al borrar tablas
            migrationBuilder.Sql("DROP TABLE IF EXISTS `Companies`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `PostalCode`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `City`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `State`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `Country`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `Language`;");
        }
    }
}
