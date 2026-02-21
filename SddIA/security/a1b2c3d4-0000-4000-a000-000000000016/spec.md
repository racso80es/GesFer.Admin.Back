# Rate Limiting y Prevención de Scraping

## Descripción
Al exponer servicios o APIs (por ejemplo, herramientas ligadas a LLMs de pago), la falta de controles puede desembocar en abusos y altos costes por peticiones masivas automatizadas.

## Medidas de Contención
Las medidas primarias de contención implican aplicar mecanismos de *Rate Limiting* estricto y bloqueos basados en IP [23, 24].

## Objetivo
Implementar estas defensas impide que atacantes o scripts de *scraping* hagan "bypass" de interfaces, y mitiga la saturación de servicios derivada de llamadas iterativas no humanas.
