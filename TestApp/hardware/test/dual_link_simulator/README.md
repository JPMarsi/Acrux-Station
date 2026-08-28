# Prueba de doble enlace con dos ESP32

Esta prueba usa una ESP32 como enlace ESP-NOW principal y otra como sustituto USB de la XBee.
La principal genera los paquetes oficiales de Container y PocketQube. Cada paquete sale por su
USB y se replica por ESP-NOW. La segunda ESP32 recibe esa copia y la entrega por otro puerto USB.

## Preparacion

1. Cargar `primary_espnow/primary_espnow.ino` en la primera ESP32.
2. Cargar `xbee_backup/xbee_backup.ino` en la segunda ESP32.
3. Conectar ambas por USB y anotar sus puertos COM.
4. En ACRUX elegir la primera como `ESP-NOW` y la segunda como `XBEE`, ambas a 115200.
5. Presionar `START TELEMETRY` y, opcionalmente, activar `CSV`.

## Casos de prueba

- Funcionamiento normal: C y P deben indicar `ESP-NOW`. Los paquetes duplicados de la segunda
  ESP32 no deben repetirse en graficos ni CSV.
- Failover sin desconectar USB: enviar `CMD,1234,PRIMARY,OFF` desde el comando personalizado.
  La principal deja de escribir por USB pero sigue replicando por ESP-NOW. Tras unos 2.5 segundos,
  C y P deben indicar `XBEE`.
- Recuperacion: enviar `CMD,1234,PRIMARY,ON`. Si el enlace activo ya es XBEE, la segunda ESP32
  retransmite el comando por ESP-NOW hacia la principal. Despues de cuatro paquetes principales
  consecutivos, C y P deben volver a `ESP-NOW`.
- Desconexion fisica: retirar el USB de la principal. La aplicacion debe continuar usando el respaldo.
  Al reconectar el mismo COM, ACRUX intenta reabrirlo cada dos segundos.

El broadcast ESP-NOW se usa solamente para facilitar esta prueba de laboratorio. No reemplaza la
configuracion peer-to-peer exigida para las radios de vuelo.
