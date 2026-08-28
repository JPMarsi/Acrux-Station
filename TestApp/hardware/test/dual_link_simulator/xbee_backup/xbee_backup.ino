#include <WiFi.h>
#include <esp_now.h>
#include <esp_arduino_version.h>

// Simulador de XBee: recibe por ESP-NOW la copia exacta del paquete generado
// por la ESP32 principal y lo entrega a ACRUX mediante su propio USB serial.
static const uint8_t BROADCAST_MAC[] = {0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF};
String commandBuffer;

void outputPacket(const uint8_t *data, int length) {
  if (length <= 0) return;
  const int printableLength = data[length - 1] == 0 ? length - 1 : length;
  Serial.write(data, printableLength);
  Serial.print("\r\n");
}

#if ESP_ARDUINO_VERSION_MAJOR >= 3
void onReceive(const esp_now_recv_info_t *info, const uint8_t *data, int length) {
  outputPacket(data, length);
}
#else
void onReceive(const uint8_t *mac, const uint8_t *data, int length) {
  outputPacket(data, length);
}
#endif

void setup() {
  Serial.begin(115200);
  WiFi.mode(WIFI_STA);
  if (esp_now_init() != ESP_OK) {
    Serial.println("ESP-NOW INIT ERROR");
    return;
  }
  esp_now_register_recv_cb(onReceive);
  esp_now_peer_info_t peer = {};
  memcpy(peer.peer_addr, BROADCAST_MAC, 6);
  peer.channel = 0;
  peer.encrypt = false;
  esp_now_add_peer(&peer);
  Serial.print("BACKUP READY MAC=");
  Serial.println(WiFi.macAddress());
}

void loop() {
  while (Serial.available()) {
    const char c = static_cast<char>(Serial.read());
    if (c == '\r' || c == '\n') {
      if (commandBuffer.length()) {
        esp_now_send(BROADCAST_MAC, reinterpret_cast<const uint8_t *>(commandBuffer.c_str()), commandBuffer.length() + 1);
      }
      commandBuffer = "";
    } else if (commandBuffer.length() < 120) {
      commandBuffer += c;
    }
  }
  delay(10);
}
