# Guía de Usuario de NCRust

Esta guía cubre la instalación, opciones de configuración, temas visuales, atajos de teclado y asociaciones de archivos para **NCRust**.

---

## 🚀 Instalación y Comienzo Rápido

### Requisitos de Compilación
Asegúrate de tener instalado el conjunto de herramientas de compilación de Rust:
```bash
cargo build --release
```
El ejecutable compilado estará ubicado en `target/release/ncrust` (o `ncrust.exe` en Windows).

### Inicio en Modo Dual
Puedes lanzar NCRust dentro de tu sesión de terminal actual directamente:
```bash
ncrust
```
Para forzar a NCRust a ejecutarse en su propia ventana de terminal independiente y optimizada, utiliza los scripts de lanzamiento (`run.bat` en Windows o `run.sh` en Unix/Linux) o haz doble clic en los accesos configurados.

---

## ⌨️ Tabla de Atajos de Teclado

NCRust admite varios perfiles de atajos (Vim, Norton Clásico, Moderno). A continuación se muestra la configuración por defecto de **Norton**:

### Control de Paneles y Navegación
| Tecla | Acción |
| :--- | :--- |
| `Tab` | Cambiar el foco entre el panel izquierdo y derecho. |
| `Arriba / Abajo` | Mover el cursor sobre los archivos. |
| `Re Pág / Av Pág` | Desplazar la lista de archivos una página arriba o abajo. |
| `Inicio / Fin` | Ir al primer o último elemento de la lista. |
| `Ctrl+U` | Intercambiar los directorios de los paneles izquierdo y derecho. |
| `Ctrl+H` | Mostrar u ocultar los archivos ocultos. |
| `Ctrl+R` | Forzar la recarga/actualización de las carpetas actuales. |
| `Ctrl+\` | Abrir la lista de favoritos de directorios (Hotlist). |

### Acciones de Archivos (Teclas F)
| Tecla | Acción |
| :--- | :--- |
| `F1` | Mostrar la ventana de ayuda de atajos de teclado. |
| `F2` | Abrir la barra de menú superior (Archivos, Comandos, Opciones). |
| `F3` | Abrir el Visor interno (modo texto o volcado hexadecimal). |
| `F4` | Abrir el Editor interno. |
| `F5` | Copiar los elementos seleccionados al panel pasivo. |
| `F6` | Renombrar o Mover los elementos seleccionados al panel pasivo. |
| `F7` | Crear una nueva carpeta (MkDir). |
| `F8` | Eliminar los elementos seleccionados. |
| `F9` | Guardar la configuración del sistema (Save Setup). |
| `F10` | Salir de NCRust de forma segura. |

### Selección y Marcación
| Tecla | Acción |
| :--- | :--- |
| `Insert` / `Espacio` | Seleccionar/marcar el elemento actual para operaciones en lote. |
| `+` (Teclado numérico) | Seleccionar un grupo de archivos según un patrón (ej. `*.rs`). |
| `-` (Teclado numérico) | Deseleccionar un grupo de archivos según un patrón. |
| `*` (Teclado numérico) | Invertir el estado de selección de la lista del panel activo. |

---

## ⚙️ Cuadro de Diálogo de Configuración (`F2 -> Opciones -> Configuración`)

El menú de configuración se divide en pestañas interactivas:

### Pestaña 0: Sistema
* **Delete to Recycle Bin:** Envía los archivos a la papelera del sistema en lugar de borrarlos permanentemente.
* **Use system copy routine:** Delega las copias de archivos a las rutinas del sistema operativo.
* **Sorting collation:** Configura el algoritmo de ordenamiento de nombres de archivos.
* **Treat digits as numbers:** Permite que `archivo2` aparezca antes que `archivo10`.
* **Case sensitive sort:** Activa el ordenamiento sensible a mayúsculas y minúsculas.

### Pestaña 1: Paneles
* **Show hidden and system files:** Muestra o esconde archivos ocultos y del sistema.
* **Select folders:** Permite que las marcas masivas apliquen también a las carpetas.
* **Sort folder names by extension:** Aplica reglas de ordenación por extensión a las carpetas.
* **Show ".." in root folders:** Muestra la opción de retroceso al directorio padre en las rutas raíz.
* **InfoPanel settings:** Personaliza los formatos del nombre del ordenador y de usuario.

### Pestaña 2: Interfaz
* **Clock:** Muestra un reloj en la esquina superior derecha.
* **Show key bar:** Muestra la fila de atajos F1-F10 en la base de la pantalla.
* **Always show the menu bar:** Mantiene siempre visible el menú superior.
* **Show total copy/delete progress indicator:** Activa la barra de progreso para tareas asíncronas.
* **Keybindings preset:** Elige entre los perfiles `"norton"`, `"vim"` o `"modern"`.

### Pestaña 4: Idioma y Complementos (Plugins)
* **Main language:** Cambia la traducción de la aplicación (ej. inglés o español).
* **Scan symbolic links:** Habilita el escaneo recursivo de enlaces simbólicos.

### Pestaña 5: Editor/Visor
* **Use external editor / viewer:** Mapea F3/F4 a comandos externos (ej. `vim`, `notepad`).
* **Tab size:** Configura el tamaño del tabulador en espacios.
* **Show line numbers:** Muestra números de línea en el editor.
* **Autodetect code page:** Detecta automáticamente la codificación de caracteres.

---

## 🎨 Temas y Estilos Personalizados

Los temas se configuran en formato TOML y se guardan en `%APPDATA%/ncrust/config/themes/` (Windows) o `~/.config/ncrust/themes/` (Linux/macOS).

Ejemplo de tema TOML:
```toml
[panel]
border = "Blue"
background = "Black"
file_selected = "Yellow"
file_directory = "Cyan"
file_executable = "Green"

[menu]
background = "Blue"
selected = "White"
```

Puedes cambiar tu tema activo editando el valor de la clave `theme` en tu archivo de configuración general TOML.
