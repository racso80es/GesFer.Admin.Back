using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace GesFer.Admin.Back.Infrastructure.Data.Migrations
{
    /// <inheritdoc />
    /// <summary>
    /// Migración inicial unificada: crea todas las tablas de Admin en GesFer_Admin.
    /// Incluye: AdminUsers, AuditLogs, Logs, Language, Country, State, City, PostalCode, Companies.
    /// Idempotente donde aplica (CREATE TABLE IF NOT EXISTS, procedimiento para columnas Logs).
    /// </summary>
    public partial class InitialAdmin : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            // 1. AdminUsers y AuditLogs
            migrationBuilder.CreateTable(
                name: "AdminUsers",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "char(36)", nullable: false, collation: "ascii_general_ci"),
                    Username = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    PasswordHash = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    FirstName = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    LastName = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    Email = table.Column<string>(type: "longtext", nullable: true)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    Role = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    LastLoginAt = table.Column<DateTime>(type: "datetime(6)", nullable: true),
                    LastLoginIp = table.Column<string>(type: "longtext", nullable: true)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    CreatedAt = table.Column<DateTime>(type: "datetime(6)", nullable: false),
                    UpdatedAt = table.Column<DateTime>(type: "datetime(6)", nullable: true),
                    DeletedAt = table.Column<DateTime>(type: "datetime(6)", nullable: true),
                    IsActive = table.Column<bool>(type: "tinyint(1)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_AdminUsers", x => x.Id);
                })
                .Annotation("MySql:CharSet", "utf8mb4");

            migrationBuilder.CreateTable(
                name: "AuditLogs",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "char(36)", nullable: false, collation: "ascii_general_ci"),
                    CursorId = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    Username = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    Action = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    HttpMethod = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    Path = table.Column<string>(type: "longtext", nullable: false)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    AdditionalData = table.Column<string>(type: "longtext", nullable: true)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    ActionTimestamp = table.Column<DateTime>(type: "datetime(6)", nullable: false),
                    CreatedAt = table.Column<DateTime>(type: "datetime(6)", nullable: false),
                    UpdatedAt = table.Column<DateTime>(type: "datetime(6)", nullable: true),
                    DeletedAt = table.Column<DateTime>(type: "datetime(6)", nullable: true),
                    IsActive = table.Column<bool>(type: "tinyint(1)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_AuditLogs", x => x.Id);
                })
                .Annotation("MySql:CharSet", "utf8mb4");

            // 2. Logs (Serilog) - idempotente
            migrationBuilder.Sql(@"
CREATE TABLE IF NOT EXISTS `Logs` (
    `Id` int NOT NULL AUTO_INCREMENT,
    `Level` longtext CHARACTER SET utf8mb4 NOT NULL,
    `Message` longtext CHARACTER SET utf8mb4 NOT NULL,
    `Template` longtext CHARACTER SET utf8mb4 NULL,
    `Exception` longtext CHARACTER SET utf8mb4 NULL,
    `Properties` longtext CHARACTER SET utf8mb4 NULL,
    `TimeStamp` datetime(6) NOT NULL,
    `Source` longtext CHARACTER SET utf8mb4 NULL,
    `CompanyId` char(36) CHARACTER SET ascii COLLATE ascii_general_ci NULL,
    `UserId` char(36) CHARACTER SET ascii COLLATE ascii_general_ci NULL,
    `ClientInfo` longtext CHARACTER SET utf8mb4 NULL,
    PRIMARY KEY (`Id`)
) CHARACTER SET utf8mb4;
");

            // 3. Columnas faltantes en Logs (si la tabla existía con esquema mínimo)
            var proc = @"
DROP PROCEDURE IF EXISTS add_logs_missing_columns;
CREATE PROCEDURE add_logs_missing_columns()
BEGIN
    IF (SELECT COUNT(*) FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'Logs' AND COLUMN_NAME = 'Source') = 0 THEN
        ALTER TABLE Logs ADD COLUMN Source longtext NULL CHARACTER SET utf8mb4;
    END IF;
    IF (SELECT COUNT(*) FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'Logs' AND COLUMN_NAME = 'CompanyId') = 0 THEN
        ALTER TABLE Logs ADD COLUMN CompanyId char(36) NULL COLLATE ascii_general_ci;
    END IF;
    IF (SELECT COUNT(*) FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'Logs' AND COLUMN_NAME = 'UserId') = 0 THEN
        ALTER TABLE Logs ADD COLUMN UserId char(36) NULL COLLATE ascii_general_ci;
    END IF;
END;
";
            migrationBuilder.Sql(proc);
            migrationBuilder.Sql("CALL add_logs_missing_columns();");
            migrationBuilder.Sql("DROP PROCEDURE add_logs_missing_columns;");

            // 4. Geo y Companies - idempotente
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
            // Geo y Companies (orden inverso por FKs)
            migrationBuilder.Sql("DROP TABLE IF EXISTS `Companies`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `PostalCode`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `City`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `State`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `Country`;");
            migrationBuilder.Sql("DROP TABLE IF EXISTS `Language`;");
            // Logs: no se elimina (puede estar en uso por Serilog)
            migrationBuilder.DropTable(name: "AuditLogs");
            migrationBuilder.DropTable(name: "AdminUsers");
        }
    }
}
