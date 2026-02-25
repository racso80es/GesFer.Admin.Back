# Postman — GesFer Admin Back API

Colección para validar todos los endpoints y acciones de la API Admin.

## Importar en Postman

1. Abre Postman.
2. **Import** → **File** → selecciona `GesFer.Admin.Back.API.postman_collection.json`.
3. La colección **GesFer Admin Back API** aparecerá en el panel izquierdo.

## Variables de colección

| Variable        | Valor por defecto                              | Uso |
|-----------------|-------------------------------------------------|-----|
| `baseUrl`       | `http://localhost:5010`                         | URL base de la API. |
| `internalSecret` | `dev-internal-secret-change-in-production`     | Header `X-Internal-Secret` para endpoints System/Admin. |
| `adminToken`    | *(vacío)*                                       | Se rellena al ejecutar **Auth → Login Admin** (script de test guarda el JWT). |

Puedes editarlas en la colección: **Variables** (pestaña).

## Orden recomendado para validar

1. **Health & Docs → Health** — Comprobar que la API está en marcha.
2. **Auth → Login Admin** — Obtener JWT (se guarda en `adminToken`).
3. **Countries → Get All Countries** — Listar países (usa `internalSecret`).
4. **Company → Get All Companies** — Listar empresas.
5. **Logs → Get Logs (Admin)** — Requiere `adminToken`.
6. El resto de peticiones según necesidad; para **By Id** sustituir `{{companyId}}`, `{{countryId}}`, `{{stateId}}` por Guids reales (por ejemplo desde las respuestas de Get All).

## Endpoints incluidos

- **Health & Docs:** Health, Swagger JSON.
- **Auth:** Login Admin (POST).
- **Logs:** Receive Log, Receive Audit Log, Get Logs, Purge Logs.
- **Countries:** Get All, Get By Id, Get States By Country Id.
- **States:** Get Cities By State Id.
- **Company:** Get All, Get By Name, Get By Id, Create, Update, Delete.
