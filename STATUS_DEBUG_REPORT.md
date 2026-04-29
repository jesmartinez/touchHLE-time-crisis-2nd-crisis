# Informe de Estado: Depuración de touchHLE (Time Crisis: 2nd Strike)

## Resumen del Problema
El emulador `touchHLE` se cerraba abruptamente al ejecutar *Time Crisis: 2nd Strike* durante `glDrawElements`. Se ha confirmado que el crash ocurría dentro del driver gráfico del host debido a punteros de memoria host-incompatibles.

## Hallazgos Técnicos (Estado Final - 29/04/2026)
1.  **Instrumentación:** Confirmado que el juego utilizaba punteros de memoria emulada en `glVertexPointer`.
2.  **Causa del Cierre:** El driver del host no podía acceder a la memoria emulada.
3.  **Solución Implementada:**
    - Se implementó exitosamente un mecanismo de `scratch_vbo` en `glDrawElements` para subir datos de vértices desde la memoria emulada a un buffer compatible con el driver del host.
    - Se añadieron tolerancias a llamadas de GLES inesperadas (`EnableClientState`, `glDisableClientState`, `glDisable(0x8840)`) mediante la actualización de la lógica de GLES-on-GL2.
    - Se implementaron stubs necesarios (`WeightPointerOES`, `MatrixIndexPointerOES`, `CurrentPaletteMatrixOES`) para evitar `panics` por funciones no implementadas.

## Estado
**RESUELTO:** El juego *Time Crisis: 2nd Strike* se ejecuta y renderiza correctamente sin experimentar cierres abruptos por incompatibilidad de punteros de memoria.
