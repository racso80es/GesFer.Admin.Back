namespace GesFer.Admin.Back.Application.Common.Interfaces;

public interface IAdminJwtService
{
    string GenerateAdminToken(string cursorId, string username, Guid userId);
}
