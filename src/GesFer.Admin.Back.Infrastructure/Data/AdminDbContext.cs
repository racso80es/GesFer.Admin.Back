using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Repository;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Infrastructure.Data;

public class AdminDbContext : DbContext, IApplicationDbContext
{
    public AdminDbContext(DbContextOptions<AdminDbContext> options) : base(options)
    {
    }

    public DbSet<AdminUser> AdminUsers => Set<AdminUser>();
    public DbSet<AuditLog> AuditLogs => Set<AuditLog>();
    public DbSet<Log> Logs => Set<Log>();
    public DbSet<Company> Companies => Set<Company>();
    public DbSet<City> Cities => Set<City>();
    public DbSet<State> States => Set<State>();
    public DbSet<Country> Countries => Set<Country>();
    public DbSet<Language> Languages => Set<Language>();
    public DbSet<PostalCode> PostalCodes => Set<PostalCode>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        // Aplicar configuraciones de entidades (incluyendo CompanyConfiguration)
        modelBuilder.ApplyConfigurationsFromAssembly(typeof(AdminDbContext).Assembly);

        // Configuración manual para Log (Serilog)
        modelBuilder.Entity<Log>(entity =>
        {
            entity.ToTable("Logs"); // Asegurar nombre de tabla
            entity.HasKey(e => e.Id);
            entity.Property(e => e.Id).ValueGeneratedOnAdd();
        });

        modelBuilder.ConfigureAdminEntities();
    }

    public override int SaveChanges()
    {
        ChangeTracker.UpdateAdminAuditFields();
        return base.SaveChanges();
    }

    public override Task<int> SaveChangesAsync(CancellationToken cancellationToken = default)
    {
        ChangeTracker.UpdateAdminAuditFields();
        return base.SaveChangesAsync(cancellationToken);
    }

    DbSet<Log> IApplicationDbContext.Logs => Logs;
    DbSet<AuditLog> IApplicationDbContext.AuditLogs => AuditLogs;
    DbSet<Company> IApplicationDbContext.Companies => Companies;
}
