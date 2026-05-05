# AGENTS.md — Proyecto CanSat ACRUX / UNSL

## Contexto general

Este repositorio pertenece al proyecto CanSat ACRUX / UNSL. El objetivo es desarrollar una estación terrena y software asociado para la misión CanSat 2026.

La estación terrena debe recibir, decodificar, visualizar, graficar, almacenar y comandar telemetría del CanSat en tiempo real.

El proyecto usa principalmente:

- Tauri
- Svelte / frontend web
- Rust en backend Tauri
- Comunicación serial
- Telemetría tipo CSV
- Posible integración con XBee
- Gestión documental técnica del proyecto

## Requisitos de misión relevantes

La estación terrena debe cumplir estos puntos del reglamento:

- Recibir telemetría a 1 Hz.
- Mostrar todos los datos en tiempo real.
- Guardar datos en CSV.
- Generar archivo con formato `Flight_<TEAM_ID>.csv`.
- Mostrar simultáneamente los datos, sin pestañas.
- Graficar en tiempo real:
  - altitud
  - voltaje de batería
  - corriente
  - acelerómetros
  - giroscopios
- Mostrar en tiempo real:
  - mission time
  - temperatura
  - posición GPS
  - packet count recibido
  - packet count perdido
  - estado del software de vuelo
- Permitir comandos:
  - CX ON/OFF
  - ST
  - SIM ENABLE
  - SIM ACTIVATE
  - SIM DISABLE
  - SIMP
  - CAL
  - MEC
- La interfaz debe ser legible a plena luz solar:
  - fondo claro
  - texto oscuro
  - fuentes grandes
  - trazos de gráficos visibles

## Formato oficial de telemetría

El paquete esperado tiene este orden:

TEAM_ID, MISSION_TIME, PACKET_COUNT, MODE, STATE, ALTITUDE,
TEMPERATURE, PRESSURE, VOLTAGE, CURRENT, GYRO_R, GYRO_P,
GYRO_Y, ACCEL_R, ACCEL_P, ACCEL_Y, GPS_TIME, GPS_ALTITUDE,
GPS_LATITUDE, GPS_LONGITUDE, GPS_SATS, CMD_ECHO

Puede haber datos opcionales al final.

## Reglas de trabajo para Codex

Antes de modificar código:

1. Leer la estructura del proyecto.
2. Identificar si el cambio afecta frontend, backend, comandos, telemetría o serial.
3. Explicar brevemente qué archivos se van a tocar.
4. Evitar reescrituras completas innecesarias.
5. Mantener la arquitectura modular.
6. No eliminar código sin justificarlo.
7. No cambiar nombres de campos oficiales de telemetría.
8. No romper compatibilidad con Tauri.
9. No introducir dependencias grandes sin motivo técnico.
10. Priorizar código claro, mantenible y entendible por estudiantes.

## Estilo de desarrollo

- Código simple antes que sobreingeniería.
- Separar parsing, estado, UI, comunicación serial y exportación CSV.
- Evitar lógica crítica directamente dentro de componentes visuales.
- Documentar funciones importantes.
- Mantener nombres claros.
- Usar estructuras tipadas cuando sea posible.
- Agregar datos simulados cuando todavía no haya hardware conectado.

## Objetivo actual del software

Construir una estación terrena funcional para:

1. Conectarse a un puerto serial.
2. Recibir paquetes de telemetría.
3. Parsear los campos oficiales.
4. Mostrar datos en dashboard.
5. Graficar variables críticas.
6. Enviar comandos al CanSat.
7. Guardar CSV oficial.
8. Ejecutar modo simulación desde archivo de presión.