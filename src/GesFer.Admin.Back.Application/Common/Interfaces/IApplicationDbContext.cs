using GesFer.Admin.Back.Domain.Entities;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Common.Interfaces;

public interface IApplicationDbContext
{
    DbSet<Log> Logs { get; }
    DbSet<AuditLog> AuditLogs { get; }
    DbSet<Company> Companies { get; }
    DbSet<Country> Countries { get; }
    DbSet<State> States { get; }
    DbSet<City> Cities { get; }
    DbSet<Language> Languages { get; }
    Task<int> SaveChangesAsync(CancellationToken cancellationToken = default);
}
