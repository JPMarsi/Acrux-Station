#include <Wire.h>
#include <Adafruit_Sensor.h>
#include <Adafruit_BMP280.h>
#include <Adafruit_SHT31.h>
#include <MPU6050_tockn.h>
#include <TinyGPS++.h>

// ==========================================
// CONFIGURACIÓN FÍSICA DEL PCB
// ==========================================

// ---------- GPS ----------
#define RX_GPS 23
#define TX_GPS 19

// ---------- XBEE ----------
#define RX_XBEE 16
#define TX_XBEE 17

#define BAUD_RATE 115200

// ==========================================
// UARTS
// ==========================================

HardwareSerial gpsSerial(1);
HardwareSerial xbeeSerial(2);

// ==========================================
// SENSORES
// ==========================================

TinyGPSPlus gps;
Adafruit_SHT31 sht31 = Adafruit_SHT31();
MPU6050 mpu6050(Wire);
Adafruit_BMP280 bmp;

// ==========================================
// VARIABLES
// ==========================================

long timer = 0;
int packetCount = 0;
bool telemetryEnabled = false;
String xbeeCommandBuffer = "";

// ==========================================
// SETUP
// ==========================================

void setup() {

  // Monitor Serie
  Serial.begin(115200);

  // XBee
  xbeeSerial.begin(BAUD_RATE, SERIAL_8N1, RX_XBEE, TX_XBEE);

  // GPS
  gpsSerial.begin(BAUD_RATE, SERIAL_8N1, RX_GPS, TX_GPS);

  // I2C
  Wire.begin();

  Serial.println("--- Iniciando Sistemas del CANSAT ---");

  // ======================================
  // SHT31
  // ======================================

  if (!sht31.begin(0x44)) {
    Serial.println("Error: No se encontró el sensor SHT31");
  }

  // ======================================
  // BMP280
  // ======================================

  if (!bmp.begin(0x76)) {
    Serial.println("Error: No se encontró el sensor BMP280");
  }

  // ======================================
  // MPU6050
  // ======================================

  mpu6050.begin();
  mpu6050.calcGyroOffsets(true);

  Serial.println("Sistemas inicializados.");
  Serial.println("Buscando satélites...");
  Serial.println("---------------------------------------------");
}

// ==========================================
// COMANDOS DESDE ESTACION TERRENA
// ==========================================

void handleXbeeCommand(String command) {
  command.trim();

  if (command.length() == 0) {
    return;
  }

  if (command == "CMD,1234,CX,ON") {
    telemetryEnabled = true;
    Serial.println("Telemetry enabled");
    return;
  }

  if (command == "CMD,1234,CX,OFF") {
    telemetryEnabled = false;
    Serial.println("Telemetry disabled");
    return;
  }

  if (command == "CMD,1234,CAL") {
    Serial.println("Calibration command received");
    return;
  }

  Serial.print("Unknown command: ");
  Serial.println(command);
}

void readXbeeCommands() {
  while (xbeeSerial.available() > 0) {
    char c = (char)xbeeSerial.read();

    if (c == '\n' || c == '\r') {
      if (xbeeCommandBuffer.length() > 0) {
        handleXbeeCommand(xbeeCommandBuffer);
        xbeeCommandBuffer = "";
      }
      continue;
    }

    xbeeCommandBuffer += c;

    if (xbeeCommandBuffer.length() > 80) {
      xbeeCommandBuffer = "";
    }
  }
}

// ==========================================
// LOOP
// ==========================================

void loop() {

  readXbeeCommands();

  // ======================================
  // GPS EN SEGUNDO PLANO
  // ======================================

  while (gpsSerial.available() > 0) {
    gps.encode(gpsSerial.read());
  }

  // ======================================
  // ACTUALIZAR IMU
  // ======================================

  mpu6050.update();

  // ======================================
  // ENVÍO CADA 1 SEGUNDO
  // ======================================

  if (telemetryEnabled && millis() - timer > 1000) {

    timer = millis();
    packetCount++;

    // ======================================
    // LECTURAS
    // ======================================

    float temp = sht31.readTemperature();
    float current = 0.0;

    float bmpPres = bmp.readPressure() / 100.0;
    float bmpAlt  = bmp.readAltitude(1013.25);

    float accX = mpu6050.getAccX();
    float accY = mpu6050.getAccY();
    float accZ = mpu6050.getAccZ();

    float gyroX = mpu6050.getGyroX();
    float gyroY = mpu6050.getGyroY();
    float gyroZ = mpu6050.getGyroZ();

    // ======================================
    // GPS
    // ======================================

    double lat = 0.0;
    double lng = 0.0;
    double gpsAlt = 0.0;
    int sats = 0;

    char horaGPS[12] = "00:00:00";

    if (gps.location.isValid()) {
      lat = gps.location.lat();
      lng = gps.location.lng();
    }

    if (gps.altitude.isValid()) {
      gpsAlt = gps.altitude.meters();
    }

    if (gps.satellites.isValid()) {
      sats = gps.satellites.value();
    }

    if (gps.time.isValid()) {

      sprintf(
        horaGPS,
        "%02d:%02d:%02d",
        gps.time.hour(),
        gps.time.minute(),
        gps.time.second()
      );
    }

    // ======================================
    // FORMATO CSV PARA APP
    // ======================================

    String paquete = "";

    paquete += "1234";                     // TEAM ID
    paquete += ",";

    paquete += horaGPS;                    // MISSION TIME
    paquete += ",";

    paquete += String(packetCount);        // PACKET COUNT
    paquete += ",";

    paquete += "F";                        // MODE
    paquete += ",";

    paquete += "ASCENT";                   // STATE
    paquete += ",";

    paquete += String(bmpAlt, 1);          // ALTITUDE
    paquete += ",";

    paquete += String(temp, 1);            // TEMPERATURE
    paquete += ",";

    paquete += String(bmpPres, 1);         // PRESSURE
    paquete += ",";

    paquete += "4.0";                      // VOLTAGE
    paquete += ",";

    paquete += String(current, 2);         // CURRENT
    paquete += ",";

    paquete += String(gyroX, 2);           // GYRO_R
    paquete += ",";

    paquete += String(gyroY, 2);           // GYRO_P
    paquete += ",";

    paquete += String(gyroZ, 2);           // GYRO_Y
    paquete += ",";

    paquete += String(accX, 2);            // ACCEL_R
    paquete += ",";

    paquete += String(accY, 2);            // ACCEL_P
    paquete += ",";

    paquete += String(accZ, 2);            // ACCEL_Y
    paquete += ",";

    paquete += horaGPS;                    // GPS TIME
    paquete += ",";

    paquete += String(gpsAlt, 1);          // GPS ALTITUDE
    paquete += ",";

    paquete += String(lat, 6);             // LAT
    paquete += ",";

    paquete += String(lng, 6);             // LNG
    paquete += ",";

    paquete += String(sats);               // SATS
    paquete += ",";

    paquete += "NONE";                     // CMD
    paquete += ",";

    paquete += "";                         // ECHO
    paquete += ",";

    paquete += "telemetry";                // TYPE

    // ======================================
    // ENVÍO POR XBEE
    // ======================================

    xbeeSerial.println(paquete);

    // ======================================
    // MOSTRAR EN MONITOR SERIE
    // ======================================

    Serial.println(paquete);
  }
}
