using BCrypt.Net;
using GesFer.Admin.Back.Application.Commands.User;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;
using GesFer.Admin.Back.Domain.ValueObjects;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.User;

public sealed class CreateUserHandler : IRequestHandler<CreateUserCommand, UserDto>
{
    private readonly IApplicationDbContext _context;

    public CreateUserHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<UserDto> Handle(CreateUserCommand request, CancellationToken cancellationToken)
    {
        var dto = request.Dto;

        var companyExists = await _context.Companies
            .AnyAsync(c => c.Id == dto.CompanyId && c.DeletedAt == null, cancellationToken);

        if (!companyExists)
            throw new InvalidOperationException($"No existe la CompanyId {dto.CompanyId}");

        var usernameExists = await _context.Users
            .AnyAsync(u => u.CompanyId == dto.CompanyId && u.Username == dto.Username && u.DeletedAt == null, cancellationToken);

        if (usernameExists)
            throw new InvalidOperationException($"Ya existe un usuario con Username '{dto.Username}' en la CompanyId {dto.CompanyId}");

        Email? email = null;
        if (!string.IsNullOrWhiteSpace(dto.Email))
            email = Email.Create(dto.Email);

        var passwordHash = BCrypt.Net.BCrypt.HashPassword(dto.Password, workFactor: 11);

        var user = new Domain.Entities.User
        {
            CompanyId = dto.CompanyId,
            Username = dto.Username,
            PasswordHash = passwordHash,
            FirstName = dto.FirstName,
            LastName = dto.LastName,
            Email = email,
            Phone = dto.Phone,
            Address = dto.Address,
            PostalCodeId = dto.PostalCodeId,
            CityId = dto.CityId,
            StateId = dto.StateId,
            CountryId = dto.CountryId,
            LanguageId = dto.LanguageId,
            CreatedAt = DateTime.UtcNow,
            IsActive = true
        };

        _context.Users.Add(user);
        await _context.SaveChangesAsync(cancellationToken);

        return user.ToDto();
    }
}

