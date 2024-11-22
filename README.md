Aquí tienes un README para tu repositorio de GitHub. Puedes incluir un enlace o un video del funcionamiento del proyecto en el espacio indicado.

---

# Sistema Solar en Rust 🚀🌌

Este proyecto simula un sistema solar utilizando gráficos en 3D implementados en Rust. Incluye elementos como planetas, órbitas, una nave espacial y un skybox para crear un entorno inmersivo. El sistema es interactivo y permite visualizar los movimientos orbitales de los planetas, la rotación de la nave espacial, y más.

## Características principales

- **Planetas con órbitas personalizadas**: Cada planeta tiene su propia órbita y velocidad de rotación.
- **Nave espacial interactiva**: Una nave espacial modelada en 3D que se puede observar desde diferentes ángulos.
- **Skybox**: Un fondo estelar que enmarca la escena para dar una experiencia inmersiva.
- **Simulación en tiempo real**: Movimientos fluidos de los planetas y la nave.
- **Interactividad**: Controles de cámara para explorar el sistema desde diferentes perspectivas.

## Requisitos

Para ejecutar este proyecto, necesitas:

- [Rust](https://www.rust-lang.org/) (versión 1.70 o superior)
- Un sistema operativo compatible (Linux, macOS, o Windows)
- [Cargo](https://doc.rust-lang.org/cargo/) (gestor de paquetes para Rust)

## Instalación y uso

1. Clona este repositorio:
   ```bash
   git clone <URL-del-repositorio>
   cd <nombre-del-repositorio>
   ```

2. Compila el proyecto:
   ```bash
   cargo build
   ```

3. Ejecuta la simulación:
   ```bash
   cargo run
   ```

## Controles

- **Cámara orbital**:
  - `Flechas izquierda/derecha`: Girar alrededor del sistema solar.
  - `Flechas arriba/abajo`: Ajustar el ángulo de visión vertical.
- **Zoom**:
  - Usa la rueda del ratón o las teclas `+` y `-` para acercarte o alejarte.
- **Vista cenital**:
  - Presiona `B` para alternar entre la vista cenital y la perspectiva normal.

## Video de demostración

*Muestra un video del sistema solar en funcionamiento aquí.*

https://github.com/user-attachments/assets/c245b1f7-fdc8-4747-94b0-6cc36f8ec11b


## Archivos clave del proyecto

- **`main.rs`**: Punto de entrada del programa.
- **`framebuffer.rs`**: Manejo del buffer de renderizado.
- **`vertex.rs`**: Definición de los vértices para los modelos 3D.
- **`shaders.rs`**: Sombras y cálculos de iluminación.
- **`obj.rs`**: Carga y procesamiento de modelos 3D.
- **`camera.rs`**: Control de la cámara.
- **`skybox.rs`**: Implementación del skybox.
- **`triangle.rs`**: Manejo de triángulos para renderizado.
- **`frustum.rs`**: Cálculo de visibilidad para optimización.

## Estructura del proyecto

```plaintext
.
├── src
│   ├── main.rs
│   ├── framebuffer.rs
│   ├── vertex.rs
│   ├── shaders.rs
│   ├── obj.rs
│   ├── camera.rs
│   ├── skybox.rs
│   ├── triangle.rs
│   ├── frustum.rs
│   └── fragment.rs
└── assets
    ├── modelos 3D (.obj)
    ├── texturas
    └── skybox
```

## Créditos

Desarrollado por [Sofía Velásquez](https://github.com/Sofiamishel2003). 

## Licencia

Este proyecto está licenciado bajo la [MIT License](LICENSE).

---

