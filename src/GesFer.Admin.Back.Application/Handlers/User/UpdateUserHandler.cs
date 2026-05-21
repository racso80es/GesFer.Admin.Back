using BCrypt.Net;
using GesFer.Admin.Back.Application.Commands.User;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;
using GesFer.Admin.Back.Domain.ValueObjects;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.User;

public sealed class UpdateUserHandler : IRequestHandler<UpdateUserCommand, UserDto>
{
    private readonly IApplicationDbContext _context;

    public UpdateUserHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<UserDto> Handle(UpdateUserCommand request, CancellationToken cancellationToken)
    {
        var user = await _context.Users
            .FirstOrDefaultAsync(u => u.Id == request.Id && u.DeletedAt == null, cancellationToken);

        if (user == null)
            throw new InvalidOperationException($"No se encontró el usuario con ID {request.Id}");

        // Unicidad Username por CompanyId (excluyendo el propio)
        var usernameExists = await _context.Users
            .AsNoTracking()
            .AnyAsync(u =>
                    u.CompanyId == user.CompanyId
                    && u.Username == request.Dto.Username
                    && u.Id != request.Id
                    && u.DeletedAt == null,
                cancellationToken);

        if (usernameExists)
            throw new InvalidOperationException($"Ya existe otro usuario con Username '{request.Dto.Username}' en la CompanyId {user.CompanyId}");

        Email? email = null;
        if (!string.IsNullOrWhiteSpace(request.Dto.Email))
            email = Email.Create(request.Dto.Email);

        user.Username = request.Dto.Username;
        user.FirstName = request.Dto.FirstName;
        user.LastName = request.Dto.LastName;
        user.Email = email;
        user.Phone = request.Dto.Phone;
        user.Address = request.Dto.Address;
        user.PostalCodeId = request.Dto.PostalCodeId;
        user.CityId = request.Dto.CityId;
        user.StateId = request.Dto.StateId;
        user.CountryId = request.Dto.CountryId;
        user.LanguageId = request.Dto.LanguageId;
        user.IsActive = request.Dto.IsActive;

        if (!string.IsNullOrWhiteSpace(request.Dto.Password))
        {
            user.PasswordHash = BCrypt.Net.BCrypt.HashPassword(request.Dto.Password, workFactor: 11);
        }

        user.UpdatedAt = DateTime.UtcNow;
        await _context.SaveChangesAsync(cancellationToken);

        return user.ToDto();
    }
}

