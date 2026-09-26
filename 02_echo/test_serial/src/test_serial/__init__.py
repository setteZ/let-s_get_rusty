import serial
import sys
import time

def main() -> None:
    rx = ""
    if len(sys.argv) < 2:
        print("I need a string to send, bye")
        sys.exit(1)
    with serial.Serial(port = "/dev/ttyACM0", baudrate = 115200, timeout = 1) as ser:
        ser.write(bytes(sys.argv[1] + "\n", "utf-8"))
        time.sleep(0.1)
        rx = ser.readline()
    if len(rx) != len(sys.argv[1]) + 1:
        print(f"Reception error, got '{rx.decode(encoding="utf-8")}' {len(rx)} bytes instead of {len(sys.argv[1]) + 1}")
        sys.exit(1)
    print(rx.decode(encoding="utf-8").rstrip())
