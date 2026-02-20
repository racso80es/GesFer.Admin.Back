namespace GesFer.Admin.Back.Domain.Services;

/// <summary>
/// Generador de GUIDs secuenciales optimizado para MySQL.
/// </summary>
public class MySqlSequentialGuidGenerator : ISequentialGuidGenerator
{
    private static readonly DateTime UnixEpoch = new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc);
    private static readonly Random Random = new Random();
    private static readonly object LockObject = new object();

    public Guid NewSequentialGuid() => NewSequentialGuid(DateTime.UtcNow);

    public Guid NewSequentialGuid(DateTime timestamp)
    {
        var timeSpan = timestamp.ToUniversalTime() - UnixEpoch;
        var milliseconds = (long)timeSpan.TotalMilliseconds;
        byte[] randomBytes;
        lock (LockObject)
        {
            randomBytes = new byte[10];
            Random.NextBytes(randomBytes);
        }
        byte[] guidBytes = new byte[16];
        var timestampBytes = BitConverter.GetBytes(milliseconds);
        if (BitConverter.IsLittleEndian)
            Array.Reverse(timestampBytes);
        Array.Copy(timestampBytes, 0, guidBytes, 0, 6);
        Array.Copy(randomBytes, 0, guidBytes, 6, 10);
        guidBytes[7] = (byte)((guidBytes[7] & 0x0F) | 0x40);
        guidBytes[8] = (byte)((guidBytes[8] & 0x3F) | 0x80);
        return new Guid(guidBytes);
    }

    public Guid NewSequentialGuidWithOffset(int millisecondsOffset) => NewSequentialGuid(DateTime.UtcNow.AddMilliseconds(millisecondsOffset));
}
