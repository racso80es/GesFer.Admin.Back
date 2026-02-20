using GesFer.Admin.Back.Domain.Entities;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Common.Interfaces;

public interface IApplicationDbContext
{
    DbSet<Log> Logs { get; }
    DbSet<AuditLog> AuditLogs { get; }
    DbSet<Company> Companies { get; }
    Task<int> SaveChangesAsync(CancellationToken cancellationToken = default);
}
