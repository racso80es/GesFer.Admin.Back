namespace GesFer.Admin.Domain.Services;

public interface ISequentialGuidGenerator
{
    Guid NewSequentialGuid();
    Guid NewSequentialGuid(DateTime timestamp);
    Guid NewSequentialGuidWithOffset(int millisecondsOffset);
}
