
# Snowflake: Constructor y Deconstructor

## Explicación Breve

Una *snowflake* es un identificador único basado en el tiempo, que permite generar millones de IDs sin colisiones, incluso en sistemas distribuidos.

### Componentes:

- **PID (Process ID):** Identificador de proceso. Es personalizado y opcional, útil para rastrear de qué proceso proviene cada ID.
- **MID (Machine ID):** Identificador de máquina o servidor. Asegura que dos máquinas no generen la misma *snowflake* al mismo tiempo.
- **Timestamp:** Tiempo actual en milisegundos desde una fecha base (por defecto, 01/01/2021). Se representa como un entero de 64 bits (`u64`), y se resta la fecha base para obtener un número más compacto.
- **Secuencia (Sequence):** Contador incremental para manejar múltiples IDs generados en el mismo milisegundo. Se limita a 12 bits (`0xfff`), lo que permite hasta 4096 IDs por milisegundo por máquina/proceso.

## Estructura

La *snowflake* final es una estructura de 63 bits (en total, ya que el bit 64 puede quedar reservado o sin uso) distribuida de la siguiente manera:

```
| timestamp (41 bits) | MID (5 bits) | PID (5 bits) | sequence (12 bits) |
```

### Detalles técnicos:

- **Timestamp (41 bits):** Representa hasta `2^41 = 2.199.023.255.552` milisegundos (~69 años), suficiente para largos períodos de tiempo.
- **MID (5 bits):** Hasta 32 servidores diferentes.
- **PID (5 bits):** Hasta 32 procesos por servidor.
- **Sequence (12 bits):** Hasta 4096 IDs por milisegundo por proceso.

En total: `2^63 = 9.223.372.036.854.775.808` combinaciones posibles.

## Deconstrucción

Para descomponer una *snowflake*, simplemente se hace el proceso inverso:

1. Extraer los 41 bits más altos para obtener el timestamp y sumarle la fecha base.
2. Extraer los 5 bits siguientes para el MID.
3. Luego, 5 bits para el PID.
4. Finalmente, los 12 bits más bajos representan la secuencia.
