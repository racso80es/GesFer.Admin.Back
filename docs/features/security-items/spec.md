# Especificación: Items de Seguridad (SddIA/security/)

Esta especificación detalla la estructura y el contenido de los items de seguridad que formarán parte de la base de conocimiento de SddIA, permitiendo que agentes como Security Engineer, Architect y Tekton Developer consulten y apliquen prácticas seguras.

## 1. Estructura del Módulo

Se creará la ruta `SddIA/security/` siguiendo el patrón establecido en `SddIA/patterns/`.

### 1.1 Contrato
*   `SddIA/security/security-contract.json`: Define el esquema JSON obligatorio para cada item.
*   `SddIA/security/security-contract.md`: Documentación legible del contrato.

### 1.2 Estructura de Items
Cada item de seguridad residirá en una carpeta única: `SddIA/security/<uuid>/`.

#### Archivos Requeridos:
1.  **`spec.md`**: Descripción detallada del item de seguridad, riesgos, mitigaciones y referencias. Formato Markdown.
2.  **`spec.json`**: Metadatos estructurados (ID, Título, Categoría, Tags, Metadata, Agentes Interesados).

## 2. Definición de Items (Alcance Inicial)

Se implementarán los siguientes 20 items de seguridad:

1.  **SQL Injection (Inyección SQL)**
2.  **Consultas Parametrizadas (Prepared Statements)**
3.  **JSON Web Tokens (JWT) y su Integridad**
4.  **Criptografía Asimétrica en JWT**
5.  **El Usuario como Vulnerabilidad Principal**
6.  **Centro de Operaciones de Seguridad (SOC)**
7.  **Red Team vs Pen Testing**
8.  **Bug Bounty Programs**
9.  **Formación mediante CTF (Capture The Flag)**
10. **Ataques de Supply Chain en Dependencias (npm)**
11. **Ejecución de malware vía scripts postinstall**
12. **Bash Injection en GitHub Actions**
13. **Fuga de Secretos en Repositorios y ficheros .env**
14. **Gestión de Secretos con Vaults**
15. **Cifrado Físico de Dispositivos**
16. **Rate Limiting y Prevención de Scraping**
17. **Firmas de Commits con GPG**
18. **Modo Vigilante (Vigilant Mode) en GitHub**
19. **Asegurando APIs con Helmet y CORS**
20. **Actualizaciones Automáticas con Dependabot**

## 3. Modelo de Seguridad

Todos los items de seguridad requerirán un contexto de ejecución validado por **Karma2Token**.
El campo `security_model` en el contrato referenciará a `SddIA/Tokens/karma2-token/spec.json`.

## 4. Agentes Interesados

Se mapearán los siguientes agentes según la categoría del item:
*   **Ciberseguridad / Ofensiva / Defensiva:** `security-engineer`, `auditor`, `architect`.
*   **DevSecOps / Infraestructura:** `tekton-developer`, `infrastructure-architect`, `security-engineer`.
*   **Seguridad de Aplicaciones:** `architect`, `tekton-developer`, `security-engineer`.
*   **Educación / Formación:** `security-engineer`, `auditor`.
