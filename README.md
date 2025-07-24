# GPS Path Tracker 🚁

A compact embedded system that captures and displays a user's travel path. Built on an AVR microcontroller in Embedded C, this project interfaces with GPS, an SD card, and an LCD to log and visualize movement.

---

## Dependencies

- [AVR GCC Compiler](https://www.microchip.com/en-us/tools-resources/develop/microchip-studio/gcc-compilers)

## 🚀 Features

- **Real-time GPS acquisition**: Interacts with a GPS module to obtain latitude, longitude, and timestamp data.
- **Route logging**: Writes continuous location data to an SD card, allowing offline storage and post-activity analysis.
- **Live path visualization**: Uses an LCD to present a simple UI showing current position, tracking stats, and a visual path map-marker.
- **Embedded firmware architecture**: Modular driver layers for GPS, SD card, and LCD, all managed by an AVR MCU using Embedded C for efficient performance.

---

## 📌 Project Architecture

```
🔐 GPS Driver   ➞  GPS Module
     └➞ Firmware Core (Embedded C) ➞ SD Card Interface
       └➞ LCD Driver ➞ LCD Display
                       ➞ SD Card Storage
```

- **GPS Driver**: Polls and parses NMEA sentences, extracts location and time.
- **SD Card Interface**: Handles logging packets of timestamped location data to external memory.
- **LCD UI**: Draws current position and path, intelligently scales/scrolls as the user moves.
- **MCU Logic**: Orchestrates hardware timing, driver coordination, and low-power handling when idle.

---

## 💡 Why This Matters

- **Practical embedded design**: Demonstrates full-stack development from low-level communication to user-facing display.
- **Hands-on sensor integration**: Covers interfacing real-world hardware: GPS, SD card, and LCD on AVR.
- **Portable utility**: Can be adapted for fitness, geocaching, navigation, or outdoor tracking.
