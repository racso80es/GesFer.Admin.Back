# Aclaraciones de Asunciones (Clarify)

1. ¿Qué debemos hacer si un Guid foráneo es inválido?
   - Si un objeto dependiente (como una `City` en un `State` dado) tiene un `StateId` inválido, debemos registrar un warning y hacer un `continue`, omitiendo ese registro.

2. ¿Y si el ID en el registro general es inválido?
   - De igual forma, si el campo `Id` principal está malformado, omitimos el registro completo y registramos un warning.

3. ¿Es necesario modificar los Unit Tests o Integration Tests actuales?
   - Probablemente no, asumiendo que ya proveen Guids correctos. Sin embargo, ejecutaremos todos los tests para confirmar que la aplicación sigue funcionando correctamente tras este cambio.