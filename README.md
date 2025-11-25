# Proyecto-3-graficas
# Nuevo README con implemntaciones del funcionamiento del sistema
Se agregaron estos modulos de funcionamiento, ya que logre utilizar otra computadora ya que mi computadora esta en necesidad desesperada de su servicio :(


# video: https://youtu.be/ao8KB_LhPU4



1. NUEVOS ARCHIVOS CREADOS:
- matrix.rs - Sistema de Matrices 4x4
Clase Matrix4 completa con todas las operaciones
Matrices: identity(), translation(), rotation_x/y/z(), scale()
Matrices de cámara: look_at(), perspective()
Operaciones: multiplicación matriz×matriz, matriz×vector

- pipeline.rs - Pipeline de Renderizado Completo
1. Vertex Shader → Transforma vértices con matrices MVP
2. Primitive Assembly → Ensambla triángulos desde índices
3. Rasterization → Convierte triángulos a píxeles
4. Fragment Shader → Calcula color final con iluminación


- mesh.rs - Sistema de Geometría 3D
Generadores procedurales: create_sphere(), create_cube(), create_orbit_ring()
Estructura Mesh con vértices e índices
Soporte para normales y coordenadas UV

- vec.rs - Añadido Vec4
Vec4 para coordenadas homogéneas
perspective_divide() para proyección

2. ARCHIVOS MODIFICADOS:
- renderer/mod.rs
Integración del Pipeline
Configuración de matrices View y Projection desde la cámara

- world/system.rs
Nueva función render_pipeline() que usa el pipeline 3D
Renderiza con mallas reales en lugar de círculos 2D


- app.rs
Muestra FPS y modo de renderizado
Actualiza matrices y uniforms cada frame


## README anterior
# Proyecto: Simulador de Sistema Solar en Rust (Software Renderer)

## Video de demostración

**Video**: [Ver video en YouTube](https://youtu.be/_i_KoSEI8W0)


---

##  Características principales

- **Sistema solar imaginario**:
  - 1 estrella principal (Sol).
  - 3 planetas con diferentes radios y velocidades orbitales.
  - 1 luna orbitando a uno de los planetas.
- **Plano eclíptico**:
  - Todos los cuerpos orbitan sobre el plano XZ (Y = 0), representando el plano eclíptico.
- **Órbitas circulares**:
  - Cada planeta/luna se traslada en una órbita circular alrededor de su cuerpo padre.
- **Rotación sobre su eje**:
  - Las texturas de los planetas/sol giran usando el ángulo del cuerpo, simulando rotación propia.
- **Cámara 3D**:
  - Movimiento 3D (adelante/atrás, izquierda/derecha, arriba/abajo).
  - Rotación de la vista (yaw/pitch) para observar el sistema desde distintos ángulos.
- **Skybox estelar**:
  - Fondo con gradiente y estrellas pseudo-aleatorias en el horizonte, simulando el espacio.
- **Colisiones básicas** (lógica incluida):
  - Se incluye lógica para evitar que la cámara/nave atraviese el sol, planetas y luna (se puede activar/desactivar en el código).
- **Warps instantáneos y animados**:
  - Teclas numéricas para saltar rápidamente a cada planeta.
  - Warp animado que interpola suavemente la posición de la cámara hasta el destino.

## Módulos principales 

El proyecto está organizado en varios módulos para que el código sea más claro y fácil de mantener:

- main.rs  
  Punto de entrada. Solo crea la aplicación (`App`) y llama a `run()`.

- app.rs 
  - Crea la ventana y el renderer.
  - Maneja el loop principal (update + render).
  - Coordina la cámara, el sistema solar, la nave, los warps y el skybox.

- renderer /
  Se encarga de dibujar en pantalla:
  - framebuffer.rs: guarda los píxeles en memoria.
  - mod.rs: funciones para proyectar 3D a 2D y dibujar líneas, círculos, órbitas y planetas texturizados.

- world/  
  Representa el sistema solar:
  - body.rs: define qué es un cuerpo (sol, planeta, luna).
  - system.rs: crea el sistema solar, actualiza las órbitas y calcula posiciones en el espacio 3D.

- camera.rs  
  Maneja la cámara 3D:
  - Guarda la posición y la orientación.
  - Responde a las teclas para moverse y girar.
  - Proporciona los vectores necesarios para proyectar la escena.


- warp.rs
  Controla los warps:
  - Warps instantáneos a planetas específicos.
  - Warp animado que interpola suavemente la posición de la cámara.

- skybox.rs
  Dibuja el fondo del espacio:
  - Gradiente de color.
  - Estrellas distribuidas en el horizonte.

- collision.rs 
  Lógica de colisiones:
  - Evita que la cámara/nave atraviesen el sol, los planetas o la luna.

- input.rs  
  Traduce las teclas del teclado a acciones:
  - Movimiento de la cámara.
  - Activación de warps.
  - Otras acciones de control del usuario.

- texture.rs 
  Carga las texturas desde assets/textures/:
  - Convierte las imágenes en arreglos de píxeles que el renderer puede usar.
  - Las imagenes fueron sacadas del sitio web de la NASA


# Movimiento de cámara:

W / S → Adelante / Atrás.

A / D → Izquierda / Derecha.

Q / E → Bajar / Subir (eje vertical).

# Rotación de cámara:

Flechas ← / → → Girar a la izquierda/derecha (yaw).

Flechas ↑ / ↓ → Mirar arriba/abajo (pitch).

# Warp:

1 → Warp al planeta 1.

2 → Warp al planeta 2.

3 → Warp al planeta 3.

# Para correr el proyecto
cargo run

cargo run --release
