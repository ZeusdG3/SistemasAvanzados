# Sistema Distribuido para Renderizado del Conjunto de Mandelbrot

Este proyecto implementa un sistema distribuido en Rust para el cálculo y renderizado de imágenes del conjunto de Mandelbrot en alta resolución. Un **coordinador** divide la imagen en bandas horizontales y las asigna a múltiples **workers**, quienes computan los píxeles y devuelven los resultados. La imagen final se ensambla y guarda localmente. Además, se incluye la posibilidad de simular condiciones de red realistas (latencia, pérdida de paquetes, ancho de banda limitado) mediante `tc` (traffic control) de Linux, activable por worker.

## Arquitectura

- **Coordinador**: Escucha en el puerto `8080`, genera tareas (bandas de la imagen) y las distribuye a los workers. Una vez que recibe todos los resultados, ensambla la imagen y la guarda en `docker/output/mandelbrot.png`.
- **Workers**: Se conectan al coordinador, solicitan tareas, calculan el conjunto de Mandelbrot para su banda y envían los datos de píxeles de vuelta.
- **Simulación de red**: Los workers pueden simular condiciones de red variables utilizando `tc`. Esto se activa mediante la variable de entorno `WORKER_ID`.

## Requisitos previos

- Docker Engine 24.x o superior
- Docker Compose v2 o superior
- (Opcional) Rust y Cargo para desarrollo local


## Configuración de red simulada "LOCAL"

El script `entrypoint.sh` aplica reglas `tc` sobre la interfaz `eth0` si la variable `WORKER_ID` está definida y no vacía. Los perfiles predefinidos son:

| WORKER_ID | Retardo         | Pérdida | Ancho de banda |
|-----------|-----------------|---------|----------------|
| 1         | 120ms ± 30ms    | 2%      | 20 Mbit        |
| 2         | 60ms ± 10ms     | 0%      | 50 Mbit        |
| 3         | 30ms ± 5ms      | 1%      | 30 Mbit        |
| 4         | 80ms ± 40ms     | 0%      | 10 Mbit        |
| otro      | 50ms ± 10ms     | 0%      | 100 Mbit       |

Si `WORKER_ID` no está definido, no se aplica ninguna regla (red sin limitaciones).

## Variables de entorno

- `WORKER_ID`: (solo workers) activa la simulación de red con el perfil correspondiente.
- `COORDINATOR_ADDR`: dirección del coordinador (por defecto `coordinator:8080` para pruebas locales).

# Cómo construir y ejecutar (LOCAL)

### Usando el archivo principal (`docker-compose.yml`)

1. **Clonar el repositorio** y situarse en la raíz del proyecto.
2. **Crear la imagen** del proyecto de la siguiente manera:
   ```bash
   docker-compose build
3. Levantar el coordinador junto con los workers
   ```bash
   docker-compose up

# Cómo construir y ejecutar (VPN)

### Usando el archivo principal (`docker-compose.yml`)

1. **Clonar el repositorio** y situarse en la raíz del proyecto.
3. **Crear la imagen** del proyecto de la siguiente manera:
   ```bash
   docker build -t docker algoritmo_distruibuido -f docker/Dockerfile .

#### Si queremos levantar el coordinador

1. **Levantar el contenedor** del coordinador desde el docker-compose del coordinador.
   ```bash
   docker-compose -f docker/docker-compose.coordinator.yml up

#### Si queremos levantar un worker

1. **Levantar el contenedor** del worker desde su docker-compose.
   ```bash
   docker-compose -f docker/docker-compose.worker.yml up --scale worker=4
  Usamos **--scale worker=N** para el numero de workers que queremos levantar. (máximo 4 de momento)

## Parámetros de renderizado
Los parámetros de la imagen se definen en rust/src/coordinator.rs:
- WIDTH: 7680 píxeles 
- HEIGHT: 4320 píxeles
- MAX_ITER: 500 iteraciones
- SUPERSAMPLING: 1 (desactivado; aumentar mejora calidad pero multiplica el tiempo)
- Región: centro en (-0.4049987, -0.5903320)
