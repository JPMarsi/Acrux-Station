#include <WiFi.h>
#include <esp_now.h>
#include <esp_arduino_version.h>

// Simulador del enlace principal. Genera telemetria oficial, la entrega por USB
// y replica exactamente el mismo paquete por ESP-NOW al simulador de respaldo.
static const uint8_t BROADCAST_MAC[] = {0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF};
static const uint32_t TELEMETRY_PERIOD_MS = 250; // 4 Hz por payload

uint32_t containerPacket = 0;
uint32_t pocketPacket = 0;
uint32_t lastTelemetry = 0;
bool primaryUsbEnabled = true;
String commandBuffer;

void handleCommand(String command);

#if ESP_ARDUINO_VERSION_MAJOR >= 3
void onReceive(const esp_now_recv_info_t *info, const uint8_t *data, int length) {
#else
void onReceive(const uint8_t *mac, const uint8_t *data, int length) {
#endif
  if (length > 0 && length < 121) {
    String command(reinterpret_cast<const char *>(data), length);
    handleCommand(command);
  }
}

void sendPacket(const String &packet) {
  if (primaryUsbEnabled) Serial.println(packet);
  esp_now_send(BROADCAST_MAC, reinterpret_cast<const uint8_t *>(packet.c_str()), packet.length() + 1);
}

String containerTelemetry(float timeSeconds) {
  const float altitude = 110.0f + 35.0f * sin(timeSeconds * 0.35f);
  const float pressure = 101325.0f - altitude * 11.5f;
  return "1234C," + String(timeSeconds, 3) + "," + String(++containerPacket) +
         ",0,F," + String(altitude, 1) + "," + String(pressure, 0) +
         ",24.5,7.6,185,0x00,ASCENT,NONE,,telemetry";
}

String pocketTelemetry(float timeSeconds) {
  const float altitude = 95.0f + 28.0f * sin(timeSeconds * 0.30f);
  const float latitude = -33.301600f + pocketPacket * 0.000002f;
  const float longitude = -66.337800f + pocketPacket * 0.000002f;
  pocketPacket++;
  return "1234P,F," + String(timeSeconds, 3) + "," + String(pocketPacket) +
         ",0," + String(altitude, 1) + ",23.8,100210,7.4,210,12:00:00," +
         String(altitude + 4.0f, 1) + "," + String(latitude, 6) + "," +
         String(longitude, 6) + ",9,1.2,-0.8,0.4,0.010,0.020,9.790," +
         "120,-85,310,4.82,4.79,0x03,NONE,STABLE,NONE,,telemetry";
}

void handleCommand(String command) {
  command.trim();
  command.toUpperCase();
  if (command.indexOf("PRIMARY,OFF") >= 0) {
    primaryUsbEnabled = false; // ESP-NOW sigue alimentando al respaldo.
  } else if (command.indexOf("PRIMARY,ON") >= 0) {
    primaryUsbEnabled = true;
    Serial.println("PRIMARY USB RESTORED");
  }
}

void readCommands() {
  while (Serial.available()) {
    const char c = static_cast<char>(Serial.read());
    if (c == '\r' || c == '\n') {
      if (commandBuffer.length()) handleCommand(commandBuffer);
      commandBuffer = "";
    } else if (commandBuffer.length() < 120) {
      commandBuffer += c;
    }
  }
}

void setup() {
  Serial.begin(115200);
  WiFi.mode(WIFI_STA);
  if (esp_now_init() != ESP_OK) {
    Serial.println("ESP-NOW INIT ERROR");
    return;
  }
  esp_now_peer_info_t peer = {};
  memcpy(peer.peer_addr, BROADCAST_MAC, 6);
  peer.channel = 0;
  peer.encrypt = false;
  esp_now_add_peer(&peer);
  esp_now_register_recv_cb(onReceive);
  Serial.print("PRIMARY READY MAC=");
  Serial.println(WiFi.macAddress());
}

void loop() {
  readCommands();
  const uint32_t now = millis();
  if (now - lastTelemetry >= TELEMETRY_PERIOD_MS) {
    lastTelemetry = now;
    const float seconds = now / 1000.0f;
    sendPacket(containerTelemetry(seconds));
    delay(4);
    sendPacket(pocketTelemetry(seconds));
  }
}
