Actúa como un Desarrollador Senior de Backend. Tu misión es implementar la entidad "User" en este microservicio (api-admin), convirtiéndolo en la fuente de verdad absoluta para la gestión de usuarios.

REQUISITO PREVIO: Este proyecto ya cuenta con las entidades 'Company' y los catálogos geográficos (PostalCode, City, State, Country, Language). Debes establecer relaciones de clave foránea (FK) directas con ellas.

Sigue esta especificación técnica detallada:

1. Definición de la Entidad (Domain):
   - Crea la clase 'User' heredando de 'BaseEntity'.
   - Campos:
     * Id: Guid (PK).
     * CompanyId: Guid (FK a Company, OnDelete: Restrict).
     * Username: string (Max 100, Obligatorio).
     * PasswordHash: string (Max 500, Obligatorio).
     * FirstName/LastName: string (Max 100, Obligatorio).
     * Email: Implementar usando el Value Object 'Email' existente (Max 200).
     * Phone/Address: string (Max 50/500, Opcionales).
     * Foreign Keys Geográficas: PostalCodeId, CityId, StateId, CountryId, LanguageId (Guids, Opcionales, OnDelete: Restrict).
     * Auditoría: CreatedAt, UpdatedAt, DeletedAt, IsActive (Soft Delete).

2. Configuración de Persistencia (EF Core - Fluent API):
   - Configura un índice único compuesto: [CompanyId, Username].
   - Define las relaciones con los catálogos geográficos y Company.
   - Asegura que el mapeo de tipos coincida con la especificación legacy (nvarchar, datetime2).

3. Capa de Aplicación (DTOs y Handlers):
   - Crea 'UserDto', 'CreateUserDto' y 'UpdateUserDto' replicando los campos del modelo legacy.
   - Implementa Handlers para CRUD completo:
     * Create/Update: Incluir lógica de hashing de contraseñas con BCrypt (WorkFactor: 11).
     * Create: Validar que el CompanyId proporcionado sea válido directamente contra la tabla local de Companies.
     * GetAll/GetById: Aplicar filtros de IsActive y Soft Delete. Asegurar aislamiento por CompanyId.

4. Controladores (API):
   - Implementa 'UserController' en la ruta '/api/User'.
   - Los endpoints deben ser idénticos en firma a los del legacy:
     * GET /api/User
     * GET /api/User/{id}
     * POST /api/User
     * PUT /api/User/{id}
     * DELETE /api/User/{id} (Debe ejecutar Soft Delete).

5. Integración de Seeds:
   - Actualiza el sistema de seeding para incluir usuarios de prueba vinculados a las empresas y localizaciones geográficas existentes en los seeds de este proyecto. Usa el hash BCrypt estático para tests: $2a$11$IRkoFxAcLpHUIwLTqkJaHu6KYx.dgfGY.sFUIsCTY9xHPhL3jcpgW

Genera el código completo siguiendo las mejores prácticas de Clean Architecture y enviando la confirmación cuando la estructura de base de datos esté lista para la migración.
