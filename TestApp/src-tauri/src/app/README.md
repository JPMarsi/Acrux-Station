# Refactorizacion de src

# Qué SI va a ir acá:
- Todo lo que sea relacionado a la interfaz
- Interfaz de linea de comando
- Asyncs o bucles (tareas)

# Qué NO va a ir acá:
- Nada de llamadas directas al serial
- Nada de guardar archivos directamente

# Que hay:
- commands
- parser
- protocol
- serial_runtime
- state
- telemetry

### Nota por si la palmo de tanto café:
- Sólo yo, la cafeina y esta larga y fría noche
- serán testigos de todas las líneas de código
- posteriormente presentes en el directorio
- sólo así mi muerte no será en vano

# Runtimes (nuevo) para que quede más organizado: 
- app_runtime: todo lo que sea de la app pero que no sea lo de abajo
- ui_runtime: todos los comandos al presionar botones o interactuar con la app
- cli_runtime: todo lo que es por command-line (version consolizada de la app, algo así como la version minimal)
- serial_runtime: todo lo que sea enviar comandos por serial (basicamente llamad al modulo y enviar lo que se tenga que enviar o si se debe ejecutar llamadas en bucle)
- telemetry_runtime: todo lo que sea leer y manejar data (esto es para guardar en archivos y mostrar en los graficos de data cargada en ram o cualquier cosa que se quiera hacer con los datos)

## app_runtime

## ui_runtime

## cli_runtime

## serial_runtime

## telemetry_runtime
