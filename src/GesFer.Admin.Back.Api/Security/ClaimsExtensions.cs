using System.Security.Claims;

namespace GesFer.Admin.Back.Api.Security;

public static class ClaimsExtensions
{
    public static bool TryGetCompanyId(this ClaimsPrincipal user, out Guid companyId)
    {
        companyId = default;

        // soporte flexible: "CompanyId" o "companyId"
        var raw = user.FindFirst("CompanyId")?.Value
                  ?? user.FindFirst("companyId")?.Value;

        return Guid.TryParse(raw, out companyId);
    }
}

