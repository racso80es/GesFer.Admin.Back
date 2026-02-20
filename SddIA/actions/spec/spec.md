# Action: Spec

## Propósito
La acción **spec** (especificación) constituye el punto de entrada formal del ciclo de desarrollo. Su objetivo es transformar requerimientos brutos, ideas iniciales o necesidades de negocio en Especificaciones Técnicas Formales (SPECS) estructuradas. Proporciona el "Qué" de forma inequívoca, estableciendo la base sobre la cual actuarán las fases de clarificación y planificación.

## Implementación
Esta acción se implementa mediante el comando `GesFer.Console --spec`.

### Sintaxis
```bash
dotnet run --project src/Console/GesFer.Console.csproj -- --spec --token <AUDITOR_TOKEN> --title <TITLE> --input <CONTENT> [--context <OUTPUT_PATH>]
```

### Argumentos
*   `--token`: Token de autorización del auditor (`AUDITOR-PROCESS`).
*   `--title`: Título breve y descriptivo de la especificación (se usará en el nombre del archivo).
*   `--input`: Contenido inicial, descripción o requerimientos brutos de la especificación.
*   `--context`: **Parámetro de entrada obligatorio para la ruta de los ficheros.** Ruta base donde se generará el archivo de especificación. **Ha de venir dada por el agente documental** (Cúmulo). La acción Spec **solo decide el nombre del fichero** (ej. `SPEC-{SanitizedTitle}.md`); la ruta completa es `{context}/{NombreFichero}.md`.
    *   **Para fixes:** La ruta de salida viene de Cúmulo (paths.fixPath/<nombre_fix>/). El invocador obtiene la ruta del agente documental (Cúmulo) y la pasa como `--context`.
    *   Para features: Cúmulo (paths.featurePath/<nombre_feature>). Si no se proporciona `--context`, el proceso debe fallar o advertir.

### Flujo de Ejecución
1.  **Validación de Token:** Verificación de identidad mediante el token del auditor (`AUDITOR-PROCESS`) para autorizar la creación de activos documentales.
2.  **Ingesta y Análisis:** Procesamiento de la entrada (`--input`) para identificar entidades, flujos de datos y requisitos funcionales.
3.  **Determinación de Contexto:** La ruta de salida es **exclusivamente** el parámetro `--context` proporcionado por el invocador. En un fix, el invocador usa paths.fixPath/<nombre_fix> (consultar Cúmulo). No hay valor por defecto; la ruta ha de venir dada.
4.  **Normalización OpenSpecs:** Aplicación de plantillas estándar para asegurar que el documento contenga las secciones obligatorias: Contexto, Arquitectura, Seguridad y Criterios de Aceptación.
5.  **Escaneo de Seguridad Inicial:** El `SecurityScanner` evalúa si los requisitos propuestos introducen riesgos de diseño o vulnerabilidades teóricas.
6.  **Persistencia:**
    *   **Markdown (.md):** Generado en `{Context}/{NombreFichero}.md`. El proceso solo decide el nombre del fichero (ej. `SPEC-admin-back-repeated-failures.md`); `Context` es el parámetro de entrada (ruta indicada por el agente documental).
    *   **Metadata JSON:** Generación de un manifiesto técnico para el rastreo de dependencias por otros agentes.
7.  **Auditoría:** Registro de la creación del documento en paths.auditsPath + paths.accessLogFile.

## Integración con Agentes
*   **Cúmulo (agente documental):** Proporciona la ruta de la tarea: para fixes paths.fixPath/<nombre_fix>; para features paths.featurePath/<nombre_feature>. La acción Spec acepta esa ruta como parámetro `--context`.
*   **Spec Architect:** Invoca esta acción con `--context` obtenido del agente documental; formaliza nuevas tareas o fixes.
*   **Clarification Specialist:** Consume el output de esta acción para iniciar el proceso de detección de "gaps".
*   **Tekton Developer:** Utiliza la especificación resultante como marco legal para la implementación del código.

## Estándares de Calidad
*   **Grado S+:** Garantiza la trazabilidad total desde el requerimiento inicial hasta el archivo persistido.
*   **Zero-Ambiguity Rule:** El proceso falla si no se definen claramente los límites del sistema (Scope).
*   **Naming Convention:** El proceso solo decide el nombre del fichero (ej. `SPEC-{SanitizedTitle}.md` o `SPEC-{bug-id}.md`). La ruta (directorio) viene dada por el parámetro `--context` proporcionado por el invocador a partir del agente documental.
