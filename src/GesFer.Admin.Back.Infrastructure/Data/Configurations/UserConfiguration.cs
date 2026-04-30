using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Domain.ValueObjects;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;

namespace GesFer.Admin.Back.Infrastructure.Data.Configurations;

public sealed class UserConfiguration : IEntityTypeConfiguration<User>
{
    private static Email? ConvertStringToEmail(string? value)
    {
        if (string.IsNullOrWhiteSpace(value))
            return null;

        return Email.TryCreate(value, out var email) ? email : (Email?)null;
    }

    public void Configure(EntityTypeBuilder<User> builder)
    {
        builder.ToTable("Users");

        builder.HasKey(u => u.Id);

        builder.Property(u => u.CompanyId)
            .IsRequired();

        builder.Property(u => u.Username)
            .IsRequired()
            .HasMaxLength(100);

        builder.Property(u => u.PasswordHash)
            .IsRequired()
            .HasMaxLength(500);

        builder.Property(u => u.FirstName)
            .IsRequired()
            .HasMaxLength(100);

        builder.Property(u => u.LastName)
            .IsRequired()
            .HasMaxLength(100);

        builder.Property(u => u.Email)
            .HasMaxLength(200)
            .HasConversion(
                email => email.HasValue ? email.Value.Value : null,
                value => ConvertStringToEmail(value));

        builder.Property(u => u.Phone)
            .HasMaxLength(50);

        builder.Property(u => u.Address)
            .HasMaxLength(500);

        builder.HasIndex(u => new { u.CompanyId, u.Username })
            .IsUnique();

        builder.HasIndex(u => u.PostalCodeId);
        builder.HasIndex(u => u.CityId);
        builder.HasIndex(u => u.StateId);
        builder.HasIndex(u => u.CountryId);
        builder.HasIndex(u => u.LanguageId);

        builder.Ignore(u => u.PostalCode);
        builder.Ignore(u => u.City);
        builder.Ignore(u => u.State);
        builder.Ignore(u => u.Country);
        builder.Ignore(u => u.Language);
    }
}

