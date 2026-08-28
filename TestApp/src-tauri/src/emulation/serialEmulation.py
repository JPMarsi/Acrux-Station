# Simulacion la telemetría por USB serial
# Raspberry pi pico RP2040
# Micropython

import sys
import time
import select
import random

TEAM_ID = 1234

telemetry_enabled = False
container_packet_count = 0
pocketqube_packet_count = 0
command_count = 0


def mission_time():
    t = time.localtime()
    return "%02d:%02d:%02d" % (t[3], t[4], t[5])


def generate_container_telemetry():
    global container_packet_count
    container_packet_count += 1

    altitude = round(random.uniform(0, 1000), 1)
    temperature = round(random.uniform(10, 35), 1)
    pressure = round(random.uniform(90000, 105000))
    voltage = round(random.uniform(3.7, 4.2), 1)
    current = random.randint(50, 900)

    fields = [
        f"{TEAM_ID}C", str(time.ticks_ms() / 1000), str(container_packet_count),
        str(command_count), "F", str(altitude), str(pressure), str(temperature),
        str(voltage), str(current), "0x00", "ASCENT", "NONE"
    ]
    return ",".join(fields) + ",,telemetry"


def generate_pocketqube_telemetry():
    global pocketqube_packet_count
    pocketqube_packet_count += 1

    altitude = round(random.uniform(0, 1000), 1)
    fields = [
        f"{TEAM_ID}P", "F", str(time.ticks_ms() / 1000), str(pocketqube_packet_count),
        str(command_count), str(altitude), str(round(random.uniform(10, 35), 1)),
        str(round(random.uniform(90000, 105000))), str(round(random.uniform(3.7, 4.2), 1)),
        str(random.randint(50, 900)), mission_time(), str(round(altitude + random.uniform(-5, 5), 1)),
        str(-33.0 + random.uniform(-0.01, 0.01)), str(-66.0 + random.uniform(-0.01, 0.01)),
        str(random.randint(5, 12)), str(round(random.uniform(-5, 5), 1)),
        str(round(random.uniform(-5, 5), 1)), str(round(random.uniform(-5, 5), 1)),
        str(round(random.uniform(-2, 2), 3)), str(round(random.uniform(-2, 2), 3)),
        str(round(random.uniform(8, 11), 3)), str(random.randint(-500, 500)),
        str(random.randint(-500, 500)), str(random.randint(-500, 500)),
        str(round(random.uniform(0, 5), 2)), str(round(random.uniform(0, 5), 2)),
        "0x00", "NONE", "STABLE", "NONE"
    ]
    return ",".join(fields) + ",,telemetry"


def check_serial():
    
    global telemetry_enabled

    # Comprobar si hay datos esperando en el buffer
    if select.select([sys.stdin], [], [], 0)[0]:
        # Leer la línea completa
        line = sys.stdin.readline().strip()
        
        # Limpiar caracteres nulos
        if not line:
            return

        # Separar por comas
        arg = line.split(',')
        
        # El comando mínimo es "CMD,1000,CX,ON" (4 elementos)
        if len(arg) < 3:
            print(f"RECIBIDO: {line} | ERROR: Faltan argumentos")
            return

        arg_cmd  = arg[0]
        arg_team = arg[1]
        arg_type = arg[2]

        # El resto de los datos
        arg_data = arg[3:]
        
        
        if arg_cmd == "CMD" and int(arg_team) == TEAM_ID:
            
            
            # CX : telemetry ON/OFF
            if arg_type == "CX":
                if arg_data[0] == "ON":
                    telemetry_enabled = True
                    print("Telemetry enabled")
                if arg_data[0] == "OFF":
                    telemetry_enabled = False
                    print("Telemetry disabled")
            
            # ST : [S]et [T]ime
            if arg_type == "ST":
                if len(arg_data) >= 1:
                    new_time = arg_data[0].split(':')
                    hour = new_time[0]
                    minu = new_time[1]
                    seco = new_time[2]
                    print("Time set to", hour,"hours",minu,"min",seco,"sec")
            
            # SIM : [SIM]ulation mode ON/OFF
            if arg_type == "SIM":
                if arg_data[0] == "ON":
                    print("Simulation: Enabled")
                if arg_data[0] == "OFF":
                    print("Simulation: Disabled")
            
            # SIMP : [SIM]ulated [P]ressure
            if arg_type == "SIMP":
                if len(arg_data) >= 1:
                    sim_pressure = arg_data[0]
                    print("Pressure simulated:", sim_pressure)
            
            # CAL : [C]ibrate [AL]titude zero
            if arg_type == "CAL":
                print("Calibrate to zero")
            
            # MEC : [ME]hanism [C]ontrol
            if arg_type == "MEC":
                if len(arg_data) >= 2:
                    device = arg_data[0]
                    state = arg_data[1]
                    
                    if state == "ON":
                        print("Mech ", device, ": Enabled")
                    if state == "OFF":
                        print("Mech ", device, ": Disabled")
            
            # OPT : [OPT]ional commands 
            if arg_type == "OPT":
                if len(arg_data) >= 1:
                    print("OPT: ", arg_data)
            
        else:
            print("CMD or TEAM not match")
            print(arg)
        
while True:
    
    check_serial()
    
    if telemetry_enabled:
        print(generate_container_telemetry())
        print(generate_pocketqube_telemetry())
    
    time.sleep(1.0)
