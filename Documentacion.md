# Documentación del Proyecto

## 1. Introducción
A lo largo de estas primeras cuatro semanas de clases, hemos aprendido conceptos basicos del origen del Sistema Operativo hasta poder llegar al Shell, con ello aprendimos el concepto de las syscalls (*llamadas al sistema*). Las syscall emplean instrucciones especiales las cuales permiten al procesador transferir el control a un codigo privilegiado. 
Con el fin de probar a mayor profundidad los conceptos de las syscall, se va a desarrollar un proyecto en el lenguaje Rust que registre en una lista todas las syscall de un programa cualquiera denominado `Prog`, el cual a la hora de ser ejecutado en la terminal con el formato `rastreador [-v | -V ] Prog [args...]`, siendo `-v` y `-V` opciones del rastreador que permiten desblegar el orden de llamadas de forma automatica o pausada del programa y `args...` se refiere a los argumentos necesarios para que el programa `Prog` funcione.
## 2. Ambiente de Desarrollo
El proyecto fue implementado en **Rust** (edition 2024) utilizando las siguientes herramientas:
- **Cargo**: Sistema de compilación y gestión de dependencias de Rust.
- **Editor/IDE**: Visual Studio Code
- **Sistema Operativo**: Linux Ubuntu
- **Compilador**: `rustc 1.89.0(29483883e 2025-08-04)` 
- **Control de versiones**: Git + GitHub

## 3. Estructuras de Datos Usadas y Funciones
- **Archivo `syscalls.json`**  
  Contiene la base de datos de syscalls, con su número, nombre, parámetros y archivo fuente dentro del kernel de Linux.

- **Funciones principales en `main.rs`**  
  - `main()`: Punto de entrada del programa. Carga el archivo `syscalls.json`, procesa su contenido y muestra los resultados.  


- **Estructuras de datos**  
  - Uso de **`serde_json`** para deserializar el archivo `syscalls.json` en estructuras de Rust.  
  - Uso de colecciones estándar **`HashMap`** para almacenar y manipular la información.  
  - **`Command`** que es una estructura de datos de **`process`** la cual permite un control muy fino en como deberia ser creado un proceso.
  - **`Verbosity`** el cual es un enum propio que se comporta a forma de flag a la hora de distinguir cuando el programa fue invocado en modo pausado, desplegando el orden de llamadas automaticamente o solo dar la tabla de resultado.
 - **Macros**
	 - **`verbosity_type!`** es una macro propia para comprobar rapidamente si el primer argumento al llamar a **rastreador** se trataba de `-v`, `-V` o si es el programa directamente.
	 - **`matches!`** es una macro del sistema utilizada en este contexto para indicarle a los if cual valor de `Verbosity` esta siendo usado.
## 4. Instrucciones para Ejecutar el Programa
Desde el directorio relativo **Rastreador**.
### Compilación
```bash
cargo build --release
```
### Ejecución
Crear archivo sin ver orden de pasos:
```bash
./target/release/rastreador /usr/bin/touch file.txt
```
Escribir en archivo viendo el orden de pasos:
```bash
./target/release/rastreador -v /usr/bin/bash -c "echo 'Hola mundo' > file.txt"
```
Borrar archivo viendo el orden de pasos de forma pausada (para pasar de un paso a otro hay que presionar la tecla `Enter`):
```bash
./target/release/rastreador -V /bin/rm file.txt
```
## 5. Actividades Realizadas por Estudiante
|Fecha                |Actividad Realizada                         |Tiempo  Invertido                         |
|----------------|-------------------------------|-----------------------------|
|26/08/2025|`Lectura de las instrucciones de la tarea`            |25 mins            |
|27/08/2025          |`"Instalacion de Rust"`            |7 mins          |
|27/08/2025          |`Aprendisaje basico de sintaxis en Rust y creacion de proyectos con cargo`|25 mins|
|27/08/2025          |`Desarrollo del componente principal del proyecto y decorado de la salida`|2 h 29 mins|
|27/08/2025          |`Creacion del repositorio y publicar el trabajo realizado en GitHub`|20 mins|
|28/08/2025          |`Realizacion de pruebas de funcionalidad e insercion de codigos de Syscalls faltantes`|10 mins|
|28/08/2025          |`-Escritura de la documentacion`|2 h|
**Todal de Horas:** 5h 31 mins
## 6. Autoevaluación
- **Estado Final del Programa:** El programa cumple con las especificaciones dadas en las instrucciones de la tarea 1.
- **Problemas Encontrados:** No es muy comun encontrar informacion en redes sobre lo pedido en la tarea, principalmente se pueden encontrar programas hechos en C o el uso de una libreria especifica de la misma.
- **Limitaciones Adicionales:** No se encontraron mayores dificultades a la hora de trabajar en la tarea.
- **Reporte de Git:**
```bash
git log
commit b2c26d52b0635737b3a3a3b05bfe41ec631fed77 (HEAD -> master, origin/master)
Author: Ion Dolanescu <idolanescu@estudiantec.cr>
Date:   Thu Aug 28 21:36:35 2025 -0600

    All the Linux syscalls added and basics manual tests realized

commit 6d5928cf1083feceacb2a61d4d948630400d3a74
Author: Ion Dolanescu <idolanescu@estudiantec.cr>
Date:   Wed Aug 27 16:46:52 2025 -0600

    Project created + verbosity filter developed
```
 - **Autoevaluacion Segun Rubrica:**
	 - *<ins>Opción -v:</ins>* 10/10
	 - *<ins>Opción -V:</ins>* 20/20
	 - *<ins>Ejecución de Prog</ins>:* 30/30
	 - *<ins>Análisis de Syscalls</ins>:* 25/30
	 - *<ins>Documentacion</ins>:* 20/20
## 7. Lecciones Aprendidas
Esta tarea permite empezar con el uso de Rust y entender un poco mejor como funcionan los forks de programas, las syscall y como crear un proyecto con cargo.
## 8. Bibliografia
 - Elhage, N. (2010, August 29). _Write yourself an strace in 70 lines  
   of code_. Made of Bugs.      
   https://blog.nelhage.com/2010/08/write-yourself-an-strace-in-70-lines-of-code/
 - Enums and pattern matching - the Rust programming language_. (n.d.). 
   https://doc.rust-lang.org/book/ch06-00-enums.html

- How do I conditionally check if an enum is one variant or another?_ (n.d.).    Stack Overflow.       https://stackoverflow.com/questions/51429501/how-do-i-conditionally-check-if-an-enum-is-one-variant-or-another    
- Valsorda, F. (n.d.). _Searchable Linux Syscall Table for x86_64_.       https://filippo.io/linux-syscall-table/  
- W3Schools.com_. (n.d.).    https://www.w3schools.com/rust/
