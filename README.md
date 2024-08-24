# ESP32C6 Bare Metal Distance Measurement Project

This project uses an ESP32C6 microcontroller to measure the distance to an object using an HC-SR04 ultrasonic sensor. The measured distance is then displayed on a 0.96" OLED screen (SSD1306 driver). The project is implemented in Rust using the `Embassy` framework for bare-metal development.

## Features

- **Ultrasonic Distance Measurement**: Utilizes the HC-SR04 sensor to calculate the distance to an object.
- **OLED Display**: The measured distance is displayed on a 0.96" OLED screen, powered by the SSD1306 driver.
- **Bare Metal Rust**: Written in Rust without an operating system, using the `Embassy` framework for asynchronous and low-power embedded applications.

## Hardware

### Components Used

- **ESP32C6**: The main microcontroller used in the project.
- **HC-SR04**: Ultrasonic sensor for distance measurement.
- **0.96" OLED Display**: A small display driven by the SSD1306 driver.

### Pin Connections

#### HC-SR04 Sensor

- **VCC** -> 3.3V
- **GND** -> GND
- **TRIG** -> GPIO6
- **ECHO** -> GPIO7

#### OLED Display (I2C)

- **VCC** -> 5V
- **GND** -> GND
- **SCK** -> GPIO5
- **SDA** -> GPIO4

## Software

### Dependencies

- **Rust**: The project is written in Rust. Make sure you have Rust installed and set up for embedded development.
- **Embassy**: A framework for embedded systems in Rust, focusing on async/await and low-power features.

### Project Structure

- **src/main.rs**: The main program logic. Initializes the peripherals, reads data from the HC-SR04 sensor, and displays the distance on the OLED screen.
- **Cargo.toml**: Manages dependencies and project configuration.

### Building and Flashing

1. **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/esp32c6-distance-measurement.git
    cd esp32c6-distance-measurement
    ```

2. **Build the project**:
    ```bash
    cargo build --release
    ```

3. **Flash the firmware to the ESP32C6**:
    ```bash
    cargo run --release
    ```

### Usage

After flashing the firmware, the ESP32C6 will start measuring the distance to an object using the HC-SR04 sensor. The distance will be continuously updated and displayed on the OLED screen.

## License

This project is licensed under the MIT License. See the LICENSE-MIT file for more details.

## Acknowledgements

- Thanks to the authors of the `Embassy` framework for providing a robust foundation for bare-metal Rust development.
- Special thanks to the Rust Embedded community for their valuable resources and support.

