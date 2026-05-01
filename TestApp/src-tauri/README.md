# src-tauri — Informe de arquitectura y funciones

## Objetivo general

La carpeta `src-tauri` contiene el **backend** de la aplicación.

Su responsabilidad es manejar todo lo que no corresponde al frontend:

- acceso al puerto serial;
- recepción de telemetría desde un emulador o una ESP32;
- interpretación de datos recibidos;
- mantenimiento del estado actual de telemetría;
- ejecución de comandos hacia el dispositivo;
- guardado de datos en archivos;
- comunicación con el frontend.

En términos prácticos, `src-tauri` es el puente entre el hardware o emulación y la interfaz visual.

---

## Estructura general

```text
src-tauri/
  data/
  src/
    emulation/
      serialEmulation.py
    modules/
      fileHandler.rs
      mod.rs
      serialController.rs
    prueba/
      commands.rs
      mod.rs
      parser.rs
      serial_runtime.rs
      state.rs
      telemetry.rs
    lib.rs
    main.rs
```

La estructura está dividida en capas:

- `main.rs`: arranque del ejecutable.
- `lib.rs`: configuración central de Tauri.
- `modules/`: infraestructura reutilizable.
- `prueba/`: lógica funcional de telemetría y comandos.
- `emulation/`: emulación externa del dispositivo.
- `data/`: salida persistente, principalmente archivos CSV.

---

# Relación jerárquica general

## Nivel 1

## `main.rs`

Es el punto de entrada del ejecutable.

### Función

Su única responsabilidad es arrancar la aplicación llamando a:

```rust
testapp_lib::run()
```

### Rol

No contiene lógica de negocio.  
Solo delega la ejecución al backend principal.

### Hijo directo

- `lib.rs`

---

## Nivel 2

## `lib.rs`

Es el orquestador principal del backend Tauri.

### Función

Configura y pone en marcha el backend:

- declara módulos internos;
- registra el estado compartido;
- registra comandos invocables desde el frontend;
- ejecuta la inicialización del sistema.

### Rol

Es el padre principal de la lógica backend.

### Hijos directos

- `modules`
- `prueba`

### Comunicación con el frontend

Este archivo participa en la comunicación con el frontend de dos formas:

#### Frontend → backend

Mediante `invoke_handler(...)`, que registra comandos Rust para ser llamados desde la UI.

#### Backend → frontend

No emite eventos directamente en la lógica principal, pero crea y habilita el entorno desde el cual otros módulos pueden hacerlo.

---

# Carpeta `modules/`

La carpeta `modules/` contiene módulos de infraestructura.

No definen la lógica de negocio principal, sino herramientas que otros módulos utilizan.

---

## `modules/mod.rs`

### Función

Declara los submódulos disponibles dentro de `modules`.

Generalmente expone:

- `serialController`
- `fileHandler`

### Rol

Es el padre organizativo de los módulos de infraestructura.

### Hijos directos

- `serialController.rs`
- `fileHandler.rs`

---

## `modules/serialController.rs`

### Función principal

Gestionar el puerto serial.

### Qué hace

Este archivo encapsula todo lo relacionado con entrada y salida serial:

- inicializa el puerto;
- lee líneas desde el puerto;
- detecta si una línea corresponde a telemetría;
- conserva o expone texto crudo recibido;
- escribe comandos al dispositivo.

### Responsabilidades

#### Entrada: dispositivo → backend

Recibe datos desde la ESP32 o desde el emulador serial.

Ejemplos:

- líneas de telemetría CSV;
- respuestas a comandos;
- mensajes de depuración.

#### Salida: backend → dispositivo

Envía comandos al payload o emulador.

Ejemplos:

- activar telemetría;
- desactivar telemetría;
- calibrar altitud;
- cambiar hora;
- accionar mecanismos.

### Rol jerárquico

Es un hijo de `modules`.

### Quién lo usa

Principalmente:

- `prueba/mod.rs`
- `prueba/serial_runtime.rs`
- `prueba/commands.rs`

### Importancia

Es el archivo que conecta el backend con el puerto serial real o simulado.

### Comunicación con el frontend

No se comunica con el frontend directamente.  
Su función es comunicar el backend con el puerto serial.

---

## `modules/fileHandler.rs`

### Función principal

Gestionar archivos y persistencia local.

### Qué hace

Se ocupa del manejo de archivos, especialmente CSV de telemetría.

Entre sus funciones típicas:

- definir la carpeta base de salida;
- crear archivos CSV;
- escribir líneas de telemetría;
- manejar encabezados;
- leer líneas guardadas;
- transformar datos para guardado.

### Rol jerárquico

Es otro hijo de `modules`.

### Quién lo usa

Principalmente:

- `prueba/mod.rs`
- `prueba/serial_runtime.rs`

### Importancia

No participa en la UI en tiempo real, pero sí en el registro persistente de la información recibida.

### Comunicación con el frontend

No se comunica con el frontend.  
Su responsabilidad es guardar datos localmente.

---

# Carpeta `prueba/`

Aunque el nombre sea `prueba`, esta carpeta concentra la lógica funcional del backend vinculada a telemetría y comandos.

Es la capa intermedia entre:

- infraestructura (`modules`);
- frontend, mediante comandos y eventos;
- fuente real o emulada de datos.

---

## `prueba/mod.rs`

### Función principal

Organizar y coordinar todo el subsistema de telemetría.

### Qué hace

- declara los submódulos de `prueba`;
- crea el estado inicial;
- ejecuta el `setup()` propio del sistema de telemetría.

### Durante `setup()`

Normalmente se encarga de:

- preparar la carpeta de salida;
- crear el CSV de telemetría;
- abrir el puerto serial;
- arrancar el runtime de lectura serial;
- enviar un comando inicial para habilitar transmisión, si corresponde.

### Rol jerárquico

Es el padre interno del subsistema de telemetría.

### Hijos directos

- `commands.rs`
- `parser.rs`
- `serial_runtime.rs`
- `state.rs`
- `telemetry.rs`

### Importancia

Si `lib.rs` arma la app Tauri, `prueba/mod.rs` arma el sistema de telemetría.

---

## `prueba/telemetry.rs`

### Función principal

Definir la estructura de datos `Telemetry`.

### Qué representa

Es el modelo formal de la telemetría que usa el backend.

Campos típicos:

- `team_id`
- `mission_time`
- `packet_count`
- `mode`
- `state`
- `altitude`
- `temperature`
- `pressure`
- `voltage`
- `current`
- `gyro_r`
- `gyro_p`
- `gyro_y`
- `accel_r`
- `accel_p`
- `accel_y`
- `gps_time`
- `gps_altitude`
- `gps_latitude`
- `gps_longitude`
- `gps_sats`
- `cmd_echo`
- `optional_data`

### Rol jerárquico

Es un modelo base.  
No ejecuta lógica por sí mismo.

### Quién lo usa

- `state.rs`
- `parser.rs`
- `commands.rs`
- `serial_runtime.rs`
- `mod.rs`

### Importancia

Define el formato oficial que entiende el backend y que luego puede enviarse al frontend.

---

## `prueba/state.rs`

### Función principal

Definir el estado compartido del backend.

### Qué guarda

Generalmente contiene la última telemetría válida recibida.

Normalmente se almacena bajo un `Mutex` para permitir acceso seguro desde distintos puntos del backend.

### Rol jerárquico

Es el contenedor del estado global de telemetría.

### Quién lo usa

- `lib.rs`, cuando registra el estado con `.manage(...)`;
- `commands.rs`;
- `serial_runtime.rs`.

### Importancia

Permite que el backend siempre tenga disponible la última telemetría válida sin tener que releer el puerto serial.

### Comunicación con el frontend

No comunica directamente con el frontend.  
Sirve como fuente local de verdad para comandos y consultas.

---

## `prueba/parser.rs`

### Función principal

Traducir una línea cruda de texto a una estructura `Telemetry`.

### Qué hace

Recibe una línea serial con formato CSV y la convierte en un objeto Rust tipado.

Ejemplo conceptual:

```text
1234,12:34:56,1,F,ASCENT,...,,telemetry
```

Esa línea se descompone y se transforma en una instancia de `Telemetry`.

### Rol jerárquico

Es una capa de traducción o interpretación.

### Quién lo usa

- `serial_runtime.rs`

### Importancia

Es el punto donde se valida e interpreta el formato de entrada.

Si este parser falla:

- la línea no se convierte;
- el estado no se actualiza;
- el frontend no recibe telemetría válida.

---

## `prueba/serial_runtime.rs`

### Función principal

Ejecutar el loop continuo de lectura serial y distribución de telemetría.

### Qué hace

Este archivo mantiene el proceso vivo que:

- lee una línea del puerto serial;
- verifica si la línea corresponde a telemetría;
- la parsea;
- actualiza el estado compartido;
- emite un evento al frontend;
- guarda la línea en CSV.

### Dependencias funcionales

Usa:

- `serialController.rs`
- `parser.rs`
- `state.rs`
- `fileHandler.rs`

### Rol jerárquico

Es un ejecutor de runtime.  
No inicializa el sistema, sino que lo mantiene funcionando en tiempo real.

### Quién lo arranca

- `prueba/mod.rs`, durante `setup()`.

### Importancia

Es uno de los archivos más importantes del flujo real.

### Comunicación con el frontend

Este archivo es el principal responsable de la comunicación **backend → frontend**.

Normalmente ahí ocurre algo equivalente a:

```rust
app_handle.emit("telemetry-update", telemetry)
```

Eso significa:

- Rust detecta una nueva telemetría;
- Rust la envía al frontend como evento;
- la interfaz puede actualizarse.

### Jerarquía de esta comunicación

- padre inmediato: `prueba/mod.rs`;
- entorno general: `lib.rs`;
- ejecutor directo: `prueba/serial_runtime.rs`.

---

## `prueba/commands.rs`

### Función principal

Definir comandos invocables desde el frontend.

### Qué hace

Expone funciones para que la UI pueda pedir acciones al backend.

Ejemplos típicos:

- `get_telemetry`
- `send_custom_command`

### Comunicación que resuelve

Este archivo es el canal principal de **frontend → backend**.

### Cómo funciona

El frontend llama una función Rust mediante `invoke(...)`.

Esa función está registrada en `lib.rs` usando `invoke_handler(...)`.

### Qué puede hacer

Según cómo lo armes, puede:

- devolver la telemetría actual;
- enviar comandos al dispositivo vía serial;
- cambiar estados internos;
- responder mensajes de éxito o error.

### Dependencias funcionales

Puede usar:

- `state.rs`
- `telemetry.rs`
- `serialController.rs`

### Rol jerárquico

Es un hijo de `prueba`.

### Importancia

Si `serial_runtime.rs` es el canal **backend → frontend**, entonces `commands.rs` es el canal **frontend → backend**.

---

# Carpeta `emulation/`

## `emulation/serialEmulation.py`

### Función principal

Simular el comportamiento del dispositivo real por serial.

### Qué hace

- escucha comandos que le llegan por entrada serial;
- interpreta esos comandos;
- activa o desactiva transmisión de telemetría;
- emite líneas de telemetría CSV a intervalos regulares.

### Tipos de comandos que reconoce

Según el emulador cargado, puede interpretar:

- `CX`
- `ST`
- `SIM`
- `SIMP`
- `CAL`
- `MEC`
- `OPT`

### Rol jerárquico

No forma parte del binario Rust.  
Es una fuente externa de datos para pruebas.

### Importancia

Permite probar toda la arquitectura serial sin depender todavía de la lógica real del vuelo.

### Relación con el backend

No habla con el frontend.  
Habla con `serialController.rs` a través del puerto serial.

---

# Carpeta `data/`

## `data/`

### Función principal

Guardar archivos generados por el backend.

### Qué contiene

Principalmente:

- CSV de telemetría.

### Quién la alimenta

- `fileHandler.rs`
- invocado desde `prueba/mod.rs`
- invocado desde `prueba/serial_runtime.rs`

### Importancia

Es la salida persistente del sistema.  
Permite conservar un historial de lo recibido por serial.

---

# Comunicación con el frontend

## 1. Frontend → backend

### Archivo responsable

```text
src-tauri/src/prueba/commands.rs
```

### Cómo se habilita

```text
src-tauri/src/lib.rs
```

### Mecanismo

1. El frontend llama `invoke(...)`.
2. `lib.rs` registra el comando.
3. `commands.rs` ejecuta la lógica.

### Padre e hijo

- padre: `lib.rs`;
- hijo: `prueba/commands.rs`.

---

## 2. Backend → frontend

### Archivo responsable directo

```text
src-tauri/src/prueba/serial_runtime.rs
```

### Cómo se habilita

1. `lib.rs` crea el entorno Tauri.
2. `prueba/mod.rs` arranca el runtime.
3. `serial_runtime.rs` emite el evento.

### Mecanismo

1. Se recibe una telemetría válida.
2. Se parsea.
3. Se actualiza el estado.
4. Se llama `emit(...)`.
5. El frontend recibe el evento.

### Padre e hijo

- padre general: `lib.rs`;
- padre funcional: `prueba/mod.rs`;
- hijo ejecutor: `prueba/serial_runtime.rs`.

---

## 3. Estado backend intermedio

### Archivo responsable

```text
src-tauri/src/prueba/state.rs
```

### Función

Mantener la última telemetría válida dentro del backend.

### Uso

Sirve de apoyo tanto para:

- consultas del frontend mediante `get_telemetry`;
- actualizaciones provenientes del runtime serial.

---

# Flujo completo del sistema

## Etapa 1: arranque

```text
main.rs
→ llama a lib.rs
```

## Etapa 2: configuración

```text
lib.rs
→ registra estado
→ registra comandos
→ llama a prueba::setup(...)
```

## Etapa 3: inicialización funcional

```text
prueba/mod.rs
→ prepara la carpeta de salida
→ crea archivos CSV
→ abre el puerto serial
→ arranca serial_runtime
```

## Etapa 4: runtime

```text
prueba/serial_runtime.rs
→ lee desde serialController.rs
→ detecta telemetría
→ parsea con parser.rs
→ actualiza state.rs
→ guarda con fileHandler.rs
→ emite al frontend
```

## Etapa 5: comandos desde la UI

```text
frontend
→ invoke(...)
→ prueba/commands.rs
→ posible escritura serial mediante serialController.rs
```

---

# Resumen corto por archivo

## `main.rs`

Arranque mínimo del ejecutable.

## `lib.rs`

Padre principal del backend Tauri. Registra módulos, comandos, estado y setup.

## `modules/mod.rs`

Declara y organiza los módulos de infraestructura.

## `modules/serialController.rs`

Maneja el puerto serial. Lee y escribe datos.

## `modules/fileHandler.rs`

Maneja archivos y persistencia CSV.

## `prueba/mod.rs`

Padre del subsistema de telemetría. Inicializa todo.

## `prueba/telemetry.rs`

Modelo estructurado de telemetría.

## `prueba/state.rs`

Estado compartido del backend.

## `prueba/parser.rs`

Convierte una línea serial CSV en una estructura `Telemetry`.

## `prueba/serial_runtime.rs`

Loop vivo de lectura serial, actualización de estado, guardado y emisión al frontend.

## `prueba/commands.rs`

Recibe invocaciones del frontend y ejecuta acciones o comandos.

## `emulation/serialEmulation.py`

Simulador serial del dispositivo.

## `data/`

Salida persistente del sistema, principalmente archivos CSV.

---

# Identificación final de responsables clave

## Padre principal del backend

```text
src-tauri/src/lib.rs
```

## Padre del subsistema de telemetría

```text
src-tauri/src/prueba/mod.rs
```

## Canal frontend → backend

```text
src-tauri/src/prueba/commands.rs
```

## Canal backend → frontend

```text
src-tauri/src/prueba/serial_runtime.rs
```

## Acceso al puerto serial

```text
src-tauri/src/modules/serialController.rs
```

## Persistencia de telemetría a CSV

```text
src-tauri/src/modules/fileHandler.rs
```

## Traducción de texto serial a estructura Rust

```text
src-tauri/src/prueba/parser.rs
```

## Fuente de verdad del estado actual

```text
src-tauri/src/prueba/state.rs
```
