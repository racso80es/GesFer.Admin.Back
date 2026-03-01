namespace GesFer.Admin.Back.Api.Configuration;

public class SystemAuthOptions
{
    public const string SectionName = "SystemAuth";

    public string SharedSecret { get; set; } = string.Empty;
}
