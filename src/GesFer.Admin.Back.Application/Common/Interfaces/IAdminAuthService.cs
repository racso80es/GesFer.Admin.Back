using GesFer.Admin.Back.Domain.Entities;

namespace GesFer.Admin.Back.Application.Common.Interfaces;

public interface IAdminAuthService
{
    Task<AdminUser?> AuthenticateAsync(string username, string password);
}
