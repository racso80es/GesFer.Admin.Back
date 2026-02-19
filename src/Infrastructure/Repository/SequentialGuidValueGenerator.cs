using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.ChangeTracking;
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.EntityFrameworkCore.ValueGeneration;
using Microsoft.Extensions.DependencyInjection;
using GesFer.Admin.Domain.Services;

namespace GesFer.Admin.Infrastructure.Repository;

/// <summary>
/// ValueGenerator de EF Core para generar GUIDs secuenciales.
/// </summary>
public class SequentialGuidValueGenerator : ValueGenerator<Guid>
{
    private static ISequentialGuidGenerator? _defaultGenerator;
    private static readonly object _lockObject = new object();

    public override bool GeneratesTemporaryValues => false;

    private static ISequentialGuidGenerator GetGuidGenerator(EntityEntry entry)
    {
        var infrastructure = entry.Context.Database as IInfrastructure<IServiceProvider>;
        if (infrastructure != null)
        {
            var serviceProvider = infrastructure.Instance;
            if (serviceProvider != null)
            {
                var generator = serviceProvider.GetService<ISequentialGuidGenerator>();
                if (generator != null)
                    return generator;
            }
        }
        if (_defaultGenerator == null)
        {
            lock (_lockObject)
            {
                if (_defaultGenerator == null)
                    _defaultGenerator = new MySqlSequentialGuidGenerator();
            }
        }
        return _defaultGenerator;
    }

    public override Guid Next(EntityEntry entry)
    {
        var generator = GetGuidGenerator(entry);
        return generator.NewSequentialGuid();
    }
}
