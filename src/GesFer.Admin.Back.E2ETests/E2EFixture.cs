namespace GesFer.Admin.Back.E2ETests;

/// <summary>
/// Fixture E2E: no arranca ningún servidor; asume que la API está ya en marcha en E2EContext.BaseUrl.
/// La orquestación (Run-E2ELocal.ps1) se encarga de prepare-full-env, invoke-mysql-seeds y opcionalmente la API.
/// </summary>
public sealed class E2EFixture : IDisposable
{
    public E2EFixture()
    {
        // Opcional: comprobar que la API responde al inicio de la suite (evita muchos fallos por timeout)
        // Si falla, los tests individuales fallarán de todas formas.
    }

    public void Dispose() { }
}
