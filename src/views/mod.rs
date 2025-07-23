use crate::{get_person_json_ld, get_website_json_ld};
use maud::{html, Markup, DOCTYPE};

pub fn render_hello_world() -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Electric Eclipse Style Showcase - Guía Completa de Estilos" }
                meta name="description" content="Showroom completo del tema Electric Eclipse: tipografía, componentes, formularios y más. Diseño accesible y responsive.";
                meta name="keywords" content="electric eclipse, style guide, design system, components, accessibility, dark mode";
                meta name="author" content="Tu Nombre";
                meta name="robots" content="index, follow";
                meta name="language" content="es";

                // Open Graph tags
                meta property="og:type" content="website";
                meta property="og:title" content="Styles Web - Hola Mundo";
                meta property="og:description" content="Una página de prueba de estilos con un simple Hola Mundo";
                meta property="og:locale" content="es_ES";

                // Favicon and app icons
                link rel="icon" type="image/x-icon" href="/static/favicon.ico";
                link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png";
                link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
                link rel="manifest" href="/static/site.webmanifest";
                meta name="theme-color" content="#2563eb";

                // CSP via meta tag
                meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self'; font-src 'self'; object-src 'none'; media-src 'self'; frame-src 'none'; base-uri 'self'; form-action 'self'";

                // Structured Data (JSON-LD)
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_website_json_ld()))
                }
                script type="application/ld+json" {
                    (maud::PreEscaped(&get_person_json_ld()))
                }

                // Stylesheets
                link rel="stylesheet" href="/static/css/electric-eclipse/main.css";
                link id="theme-stylesheet" rel="stylesheet" href="/static/css/electric-eclipse/light.css";

                // Scripts
                script src="/static/js/theme-init.js" defer {}
                script src="/static/js/htmx.min.js" defer {}
            }
            body {
                header {
                    div class="container" {
                        nav {
                            a class="logo" href="/" { "Styles Web" }
                            
                            div class="nav-controls" {
                                button class="theme-toggle" id="theme-toggle" aria-label="Toggle theme" {
                                    svg class="theme-icon-sun" width="20" height="20" fill="currentColor" viewBox="0 0 20 20" {
                                        path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd" {}
                                    }
                                    svg class="theme-icon-moon" width="20" height="20" fill="currentColor" viewBox="0 0 20 20" style="display: none;" {
                                        path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" {}
                                    }
                                }
                            }
                        }
                    }
                }

                main {
                    div class="container" {
                        // Hero Section
                        section class="hero-section" {
                            h1 { "Electric Eclipse Style Showcase" }
                            p class="lead" { 
                                "Una guía completa de todos los elementos y componentes disponibles en el tema Electric Eclipse. "
                                "Diseñado para accesibilidad perfecta, soporte completo de temas claro/oscuro y experiencia responsive óptima."
                            }
                        }

                        // Typography Section
                        section class="content-section" {
                            h2 class="section-title" { "1. Tipografía" }
                            
                            div class="subsection" {
                                h3 { "1.1 Encabezados" }
                                h1 { "Heading 1 - Título Principal" }
                                h2 { "Heading 2 - Sección Mayor" }
                                h3 { "Heading 3 - Subsección" }
                                h4 { "Heading 4 - Apartado" }
                                h5 { "Heading 5 - Sub-apartado" }
                                h6 { "Heading 6 - Detalle Menor" }
                            }

                            div class="subsection" {
                                h3 { "1.2 Párrafos y Texto" }
                                p { 
                                    "Este es un párrafo normal con texto estándar. Lorem ipsum dolor sit amet, consectetur adipiscing elit. "
                                    "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                                }
                                p class="lead" { 
                                    "Este es un párrafo destacado (lead) con tamaño de fuente mayor para introducir contenido importante."
                                }
                                p class="small" { 
                                    "Este es texto pequeño, útil para notas al pie, aclaraciones o información secundaria."
                                }
                            }

                            div class="subsection" {
                                h3 { "1.3 Listas" }
                                
                                h4 { "Lista no ordenada" }
                                ul {
                                    li { "Elemento de lista simple" }
                                    li { 
                                        "Elemento con sublista"
                                        ul {
                                            li { "Subelemento 1" }
                                            li { "Subelemento 2" }
                                            li { "Subelemento 3" }
                                        }
                                    }
                                    li { "Otro elemento de lista" }
                                }

                                h4 { "Lista ordenada" }
                                ol {
                                    li { "Primer paso del proceso" }
                                    li { "Segundo paso importante" }
                                    li { 
                                        "Tercer paso con detalles"
                                        ol {
                                            li { "Detalle A" }
                                            li { "Detalle B" }
                                        }
                                    }
                                }

                                h4 { "Lista de definiciones" }
                                dl {
                                    dt { "HTML" }
                                    dd { "HyperText Markup Language - Lenguaje de marcado para crear páginas web" }
                                    
                                    dt { "CSS" }
                                    dd { "Cascading Style Sheets - Hojas de estilo para diseño visual" }
                                    
                                    dt { "JavaScript" }
                                    dd { "Lenguaje de programación para interactividad web" }
                                }
                            }
                        }

                        // Inline Text Elements Section
                        section class="content-section" {
                            h2 class="section-title" { "2. Elementos de Texto Inline" }
                            
                            div class="subsection" {
                                h3 { "2.1 Estilos de Texto" }
                                p {
                                    "Texto con " strong { "énfasis fuerte (strong)" } " y texto con " em { "énfasis (em)" } ". "
                                    "También tenemos " b { "texto en negrita (b)" } " y " i { "texto en cursiva (i)" } "."
                                }
                                p {
                                    "Texto " mark { "resaltado (mark)" } ", texto " del { "eliminado (del)" } " y texto " ins { "insertado (ins)" } "."
                                }
                                p {
                                    "Texto " small { "pequeño (small)" } ", " sub { "subíndice (sub)" } " y " sup { "superíndice (sup)" } "."
                                }
                                p {
                                    "Texto " u { "subrayado (u)" } " y texto " s { "tachado (s)" } "."
                                }
                            }

                            div class="subsection" {
                                h3 { "2.2 Enlaces" }
                                p {
                                    "Un " a href="#" { "enlace normal" } ", un "
                                    a href="#" class="link-underline" { "enlace subrayado" } " y un "
                                    a href="https://example.com" target="_blank" rel="noopener" { "enlace externo" } " ↗"
                                }
                            }

                            div class="subsection" {
                                h3 { "2.3 Código y Elementos Técnicos" }
                                p {
                                    "Código inline: " code { "const greeting = 'Hola Mundo';" } " en medio del texto."
                                }
                                p {
                                    "Entrada de teclado: Presiona " kbd { "Ctrl" } " + " kbd { "C" } " para copiar."
                                }
                                p {
                                    "Variable: El valor de " var { "x" } " es " samp { "42" } " (salida de programa)."
                                }
                                p {
                                    "Abreviatura: " abbr title="World Wide Web Consortium" { "W3C" } " establece estándares web."
                                }
                            }

                            div class="subsection" {
                                h3 { "2.4 Citas" }
                                p {
                                    "Como dijo Einstein: " q { "La imaginación es más importante que el conocimiento" } "."
                                }
                                blockquote {
                                    p {
                                        "Este es un blockquote más largo para citas extensas. Lorem ipsum dolor sit amet, "
                                        "consectetur adipiscing elit. Vivamus lacinia odio vitae vestibulum."
                                    }
                                    footer { 
                                        "—" cite { "Autor Famoso" } ", " 
                                        cite { "Obra Importante" }
                                    }
                                }
                            }
                        }

                        // Forms Section
                        section class="content-section" {
                            h2 class="section-title" { "3. Formularios y Controles" }
                            
                            form {
                                div class="subsection" {
                                    h3 { "3.1 Campos de Texto" }
                                    
                                    div class="form-group" {
                                        label for="text-input" { "Campo de texto:" }
                                        input type="text" id="text-input" name="text" placeholder="Escribe algo aquí..." {}
                                    }

                                    div class="form-group" {
                                        label for="email-input" { "Email:" }
                                        input type="email" id="email-input" name="email" placeholder="tu@email.com" required {}
                                    }

                                    div class="form-group" {
                                        label for="password-input" { "Contraseña:" }
                                        input type="password" id="password-input" name="password" placeholder="••••••••" {}
                                    }

                                    div class="form-group" {
                                        label for="number-input" { "Número:" }
                                        input type="number" id="number-input" name="number" min="0" max="100" step="5" value="50" {}
                                    }

                                    div class="form-group" {
                                        label for="search-input" { "Búsqueda:" }
                                        input type="search" id="search-input" name="search" placeholder="Buscar..." {}
                                    }

                                    div class="form-group" {
                                        label for="url-input" { "URL:" }
                                        input type="url" id="url-input" name="url" placeholder="https://ejemplo.com" {}
                                    }

                                    div class="form-group" {
                                        label for="tel-input" { "Teléfono:" }
                                        input type="tel" id="tel-input" name="tel" placeholder="+34 600 000 000" {}
                                    }

                                    div class="form-group" {
                                        label for="textarea" { "Área de texto:" }
                                        textarea id="textarea" name="message" rows="4" placeholder="Escribe un mensaje largo..." {}
                                    }
                                }

                                div class="subsection" {
                                    h3 { "3.2 Selectores y Opciones" }
                                    
                                    div class="form-group" {
                                        label for="select" { "Selector desplegable:" }
                                        select id="select" name="option" {
                                            option value="" { "Selecciona una opción" }
                                            optgroup label="Grupo 1" {
                                                option value="1" { "Opción 1" }
                                                option value="2" { "Opción 2" }
                                            }
                                            optgroup label="Grupo 2" {
                                                option value="3" { "Opción 3" }
                                                option value="4" selected { "Opción 4 (seleccionada)" }
                                            }
                                        }
                                    }

                                    div class="form-group" {
                                        label for="datalist-input" { "Input con sugerencias:" }
                                        input type="text" id="datalist-input" list="suggestions" placeholder="Empieza a escribir..." {}
                                        datalist id="suggestions" {
                                            option value="JavaScript" {}
                                            option value="TypeScript" {}
                                            option value="Rust" {}
                                            option value="Python" {}
                                            option value="Go" {}
                                        }
                                    }
                                }

                                div class="subsection" {
                                    h3 { "3.3 Checkboxes y Radio Buttons" }
                                    
                                    fieldset {
                                        legend { "Checkboxes - Selección múltiple:" }
                                        
                                        div class="form-check" {
                                            input type="checkbox" id="check1" name="features" value="feature1" checked {}
                                            label for="check1" { "Opción 1 (marcada)" }
                                        }
                                        
                                        div class="form-check" {
                                            input type="checkbox" id="check2" name="features" value="feature2" {}
                                            label for="check2" { "Opción 2" }
                                        }
                                        
                                        div class="form-check" {
                                            input type="checkbox" id="check3" name="features" value="feature3" disabled {}
                                            label for="check3" { "Opción 3 (deshabilitada)" }
                                        }
                                    }

                                    fieldset {
                                        legend { "Radio buttons - Selección única:" }
                                        
                                        div class="form-check" {
                                            input type="radio" id="radio1" name="choice" value="option1" checked {}
                                            label for="radio1" { "Opción A (seleccionada)" }
                                        }
                                        
                                        div class="form-check" {
                                            input type="radio" id="radio2" name="choice" value="option2" {}
                                            label for="radio2" { "Opción B" }
                                        }
                                        
                                        div class="form-check" {
                                            input type="radio" id="radio3" name="choice" value="option3" disabled {}
                                            label for="radio3" { "Opción C (deshabilitada)" }
                                        }
                                    }
                                }

                                div class="subsection" {
                                    h3 { "3.4 Controles Especiales" }
                                    
                                    div class="form-group" {
                                        label for="date-input" { "Fecha:" }
                                        input type="date" id="date-input" name="date" {}
                                    }

                                    div class="form-group" {
                                        label for="time-input" { "Hora:" }
                                        input type="time" id="time-input" name="time" {}
                                    }

                                    div class="form-group" {
                                        label for="datetime-input" { "Fecha y hora local:" }
                                        input type="datetime-local" id="datetime-input" name="datetime" {}
                                    }

                                    div class="form-group" {
                                        label for="month-input" { "Mes:" }
                                        input type="month" id="month-input" name="month" {}
                                    }

                                    div class="form-group" {
                                        label for="week-input" { "Semana:" }
                                        input type="week" id="week-input" name="week" {}
                                    }

                                    div class="form-group" {
                                        label for="color-input" { "Color:" }
                                        input type="color" id="color-input" name="color" value="#6d28d9" {}
                                    }

                                    div class="form-group" {
                                        label for="range-input" { "Rango (0-100):" }
                                        input type="range" id="range-input" name="range" min="0" max="100" value="75" {}
                                        output for="range-input" { "75" }
                                    }

                                    div class="form-group" {
                                        label for="file-input" { "Archivo:" }
                                        input type="file" id="file-input" name="file" accept="image/*,.pdf" {}
                                    }
                                }

                                div class="subsection" {
                                    h3 { "3.5 Estados de Formulario" }
                                    
                                    div class="form-group" {
                                        label for="disabled-input" { "Campo deshabilitado:" }
                                        input type="text" id="disabled-input" value="No se puede editar" disabled {}
                                    }

                                    div class="form-group" {
                                        label for="readonly-input" { "Campo de solo lectura:" }
                                        input type="text" id="readonly-input" value="Solo lectura" readonly {}
                                    }

                                    div class="form-group error" {
                                        label for="error-input" { "Campo con error:" }
                                        input type="email" id="error-input" class="error" value="email-invalido" aria-invalid="true" aria-describedby="error-msg" {}
                                        span id="error-msg" class="error-message" { "Por favor ingresa un email válido" }
                                    }

                                    div class="form-group success" {
                                        label for="success-input" { "Campo válido:" }
                                        input type="text" id="success-input" class="success" value="Campo correcto" aria-invalid="false" {}
                                        span class="success-message" { "✓ Validación exitosa" }
                                    }
                                }
                            }
                        }

                        // Buttons Section
                        section class="content-section" {
                            h2 class="section-title" { "4. Botones y Acciones" }
                            
                            div class="subsection" {
                                h3 { "4.1 Variantes de Botones" }
                                
                                div class="button-group" {
                                    button class="btn btn-primary" { "Primario" }
                                    button class="btn btn-secondary" { "Secundario" }
                                    button class="btn btn-success" { "Éxito" }
                                    button class="btn btn-danger" { "Peligro" }
                                    button class="btn btn-warning" { "Advertencia" }
                                    button class="btn btn-info" { "Información" }
                                }
                            }

                            div class="subsection" {
                                h3 { "4.2 Tamaños de Botones" }
                                
                                div class="button-group" {
                                    button class="btn btn-primary btn-sm" { "Pequeño" }
                                    button class="btn btn-primary" { "Normal" }
                                    button class="btn btn-primary btn-lg" { "Grande" }
                                }
                            }

                            div class="subsection" {
                                h3 { "4.3 Estados de Botones" }
                                
                                div class="button-group" {
                                    button class="btn btn-primary" { "Normal" }
                                    button class="btn btn-primary" disabled { "Deshabilitado" }
                                    button class="btn btn-primary loading" { "Cargando..." }
                                }
                            }

                            div class="subsection" {
                                h3 { "4.4 Botones Especiales" }
                                
                                div class="button-group" {
                                    button class="btn btn-outline" { "Outline" }
                                    button class="btn btn-ghost" { "Ghost" }
                                    button class="btn btn-link" { "Enlace" }
                                    button class="btn btn-icon" aria-label="Favorito" { "❤️" }
                                }
                                
                                div class="button-group" {
                                    a href="#" class="btn btn-primary" role="button" { "Enlace como botón" }
                                    input type="submit" class="btn btn-secondary" value="Submit Input" {}
                                    input type="button" class="btn btn-info" value="Button Input" {}
                                }
                            }
                        }

                        // Tables Section
                        section class="content-section" {
                            h2 class="section-title" { "5. Tablas y Datos" }
                            
                            div class="subsection" {
                                h3 { "5.1 Tabla Básica" }
                                
                                table {
                                    caption { "Estadísticas de rendimiento web" }
                                    thead {
                                        tr {
                                            th scope="col" { "Métrica" }
                                            th scope="col" { "Valor" }
                                            th scope="col" { "Estado" }
                                            th scope="col" { "Tendencia" }
                                        }
                                    }
                                    tbody {
                                        tr {
                                            th scope="row" { "Lighthouse Score" }
                                            td { "100/100" }
                                            td { span class="badge badge-success" { "Excelente" } }
                                            td { "↑ +5" }
                                        }
                                        tr {
                                            th scope="row" { "Tiempo de Carga" }
                                            td { "1.2s" }
                                            td { span class="badge badge-warning" { "Bueno" } }
                                            td { "↓ -0.3s" }
                                        }
                                        tr {
                                            th scope="row" { "Tamaño Total" }
                                            td { "245 KB" }
                                            td { span class="badge badge-info" { "Óptimo" } }
                                            td { "= 0" }
                                        }
                                    }
                                    tfoot {
                                        tr {
                                            th scope="row" colspan="3" { "Promedio General" }
                                            td { strong { "95%" } }
                                        }
                                    }
                                }
                            }

                            div class="subsection" {
                                h3 { "5.2 Tabla Responsiva" }
                                
                                div class="table-responsive" {
                                    table class="table-striped" {
                                        thead {
                                            tr {
                                                th { "ID" }
                                                th { "Nombre" }
                                                th { "Email" }
                                                th { "Rol" }
                                                th { "Estado" }
                                                th { "Acciones" }
                                            }
                                        }
                                        tbody {
                                            tr {
                                                td { "001" }
                                                td { "Ana García" }
                                                td { "ana@example.com" }
                                                td { "Admin" }
                                                td { span class="badge badge-success" { "Activo" } }
                                                td {
                                                    button class="btn btn-sm btn-primary" { "Editar" }
                                                    " "
                                                    button class="btn btn-sm btn-danger" { "Eliminar" }
                                                }
                                            }
                                            tr {
                                                td { "002" }
                                                td { "Carlos López" }
                                                td { "carlos@example.com" }
                                                td { "Usuario" }
                                                td { span class="badge badge-secondary" { "Inactivo" } }
                                                td {
                                                    button class="btn btn-sm btn-primary" { "Editar" }
                                                    " "
                                                    button class="btn btn-sm btn-danger" { "Eliminar" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // UI Components Section
                        section class="content-section" {
                            h2 class="section-title" { "6. Componentes UI" }
                            
                            div class="subsection" {
                                h3 { "6.1 Cards" }
                                
                                div class="grid grid-cols-3" {
                                    div class="card" {
                                        div class="card-header" {
                                            h4 { "Card Básica" }
                                        }
                                        div class="card-body" {
                                            p { "Contenido de la card con información relevante." }
                                        }
                                        div class="card-footer" {
                                            button class="btn btn-primary btn-sm" { "Acción" }
                                        }
                                    }

                                    div class="card card-bordered" {
                                        div class="card-body" {
                                            h4 { "Card con Borde" }
                                            p { "Esta card tiene un borde definido para mayor separación visual." }
                                        }
                                    }

                                    div class="card card-elevated" {
                                        div class="card-body" {
                                            h4 { "Card Elevada" }
                                            p { "Card con sombra para efecto de elevación." }
                                        }
                                    }
                                }
                            }

                            div class="subsection" {
                                h3 { "6.2 Alertas" }
                                
                                div class="alert alert-info" role="alert" {
                                    strong { "Información:" } " Este es un mensaje informativo."
                                }
                                
                                div class="alert alert-success" role="alert" {
                                    strong { "¡Éxito!" } " La operación se completó correctamente."
                                }
                                
                                div class="alert alert-warning" role="alert" {
                                    strong { "Advertencia:" } " Por favor revisa esta información."
                                }
                                
                                div class="alert alert-danger" role="alert" {
                                    strong { "Error:" } " Algo salió mal. Por favor intenta de nuevo."
                                }
                            }

                            div class="subsection" {
                                h3 { "6.3 Badges y Etiquetas" }
                                
                                p {
                                    "Estados: "
                                    span class="badge" { "Default" } " "
                                    span class="badge badge-primary" { "Primary" } " "
                                    span class="badge badge-secondary" { "Secondary" } " "
                                    span class="badge badge-success" { "Success" } " "
                                    span class="badge badge-danger" { "Danger" } " "
                                    span class="badge badge-warning" { "Warning" } " "
                                    span class="badge badge-info" { "Info" } " "
                                }
                                
                                p {
                                    "Notificaciones: "
                                    span class="badge badge-pill" { "5" } " "
                                    span class="badge badge-pill badge-primary" { "Nuevo" } " "
                                    span class="badge badge-pill badge-danger" { "99+" }
                                }
                            }

                            div class="subsection" {
                                h3 { "6.4 Progress Bars" }
                                
                                div class="progress" {
                                    div class="progress-bar" role="progressbar" style="width: 25%" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100" {
                                        "25%"
                                    }
                                }
                                
                                div class="progress" {
                                    div class="progress-bar progress-bar-success" role="progressbar" style="width: 75%" aria-valuenow="75" aria-valuemin="0" aria-valuemax="100" {
                                        "75%"
                                    }
                                }
                                
                                div class="progress" {
                                    div class="progress-bar progress-bar-striped" role="progressbar" style="width: 50%" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100" {
                                        "50%"
                                    }
                                }
                            }

                            div class="subsection" {
                                h3 { "6.5 Tooltips y Popovers" }
                                
                                p {
                                    "Hover sobre estos elementos: "
                                    span class="tooltip" data-tooltip="Este es un tooltip" { "Tooltip ejemplo" } ", "
                                    button class="btn btn-secondary" data-popover="Contenido más extenso del popover" { "Popover" }
                                }
                            }
                        }

                        // Media Section
                        section class="content-section" {
                            h2 class="section-title" { "7. Elementos Multimedia" }
                            
                            div class="subsection" {
                                h3 { "7.1 Imágenes" }
                                
                                figure {
                                    img src="https://via.placeholder.com/600x400" alt="Imagen de ejemplo" class="img-fluid" {}
                                    figcaption { "Figura 1: Imagen responsiva de ejemplo" }
                                }
                                
                                div class="image-gallery" {
                                    img src="https://via.placeholder.com/200x200" alt="Miniatura 1" class="img-thumbnail" {}
                                    img src="https://via.placeholder.com/200x200" alt="Miniatura 2" class="img-thumbnail img-rounded" {}
                                    img src="https://via.placeholder.com/200x200" alt="Miniatura 3" class="img-thumbnail img-circle" {}
                                }
                            }

                            div class="subsection" {
                                h3 { "7.2 Video y Audio" }
                                
                                video controls class="video-player" {
                                    source src="movie.mp4" type="video/mp4" {}
                                    "Tu navegador no soporta el elemento video."
                                }
                                
                                audio controls class="audio-player" {
                                    source src="audio.mp3" type="audio/mpeg" {}
                                    "Tu navegador no soporta el elemento audio."
                                }
                            }

                            div class="subsection" {
                                h3 { "7.3 Embeds" }
                                
                                div class="embed-responsive" {
                                    iframe src="https://www.youtube.com/embed/dQw4w9WgXcQ" 
                                           title="Video de YouTube" 
                                           frameborder="0" 
                                           allowfullscreen {}
                                }
                            }
                        }

                        // Layout Helpers Section
                        section class="content-section" {
                            h2 class="section-title" { "8. Utilidades de Layout" }
                            
                            div class="subsection" {
                                h3 { "8.1 Grid System" }
                                
                                div class="grid grid-cols-2" {
                                    div class="grid-item" { "Columna 1/2" }
                                    div class="grid-item" { "Columna 2/2" }
                                }
                                
                                div class="grid grid-cols-3" {
                                    div class="grid-item" { "1/3" }
                                    div class="grid-item" { "2/3" }
                                    div class="grid-item" { "3/3" }
                                }
                                
                                div class="grid grid-cols-4" {
                                    div class="grid-item col-span-2" { "Span 2 columnas" }
                                    div class="grid-item" { "1 col" }
                                    div class="grid-item" { "1 col" }
                                }
                            }

                            div class="subsection" {
                                h3 { "8.2 Flexbox Utilities" }
                                
                                div class="flex justify-between" {
                                    div { "Inicio" }
                                    div { "Centro" }
                                    div { "Fin" }
                                }
                                
                                div class="flex items-center gap-4" {
                                    div class="flex-1" { "Flex 1" }
                                    div class="flex-2" { "Flex 2" }
                                    div class="flex-1" { "Flex 1" }
                                }
                            }

                            div class="subsection" {
                                h3 { "8.3 Espaciado" }
                                
                                div class="mb-2" { "Margen inferior pequeño" }
                                div class="my-4" { "Margen vertical mediano" }
                                div class="p-4" { "Padding completo" }
                                div class="px-6 py-3" { "Padding horizontal y vertical diferente" }
                            }
                        }

                        // Code Section
                        section class="content-section" {
                            h2 class="section-title" { "9. Código y Sintaxis" }
                            
                            div class="subsection" {
                                h3 { "9.1 Bloques de Código" }
                                
                                pre {
                                    code class="language-rust" {
                                        "// Ejemplo de código Rust\n"
                                        "fn main() {\n"
                                        "    let mensaje = \"¡Hola, Electric Eclipse!\";\n"
                                        "    println!(\"{}\", mensaje);\n"
                                        "    \n"
                                        "    // Iterador de ejemplo\n"
                                        "    let numeros = vec![1, 2, 3, 4, 5];\n"
                                        "    let suma: i32 = numeros.iter().sum();\n"
                                        "    println!(\"La suma es: {}\", suma);\n"
                                        "}"
                                    }
                                }
                                
                                pre {
                                    code class="language-javascript" {
                                        "// Ejemplo de código JavaScript\n"
                                        "class ThemeManager {\n"
                                        "    constructor() {\n"
                                        "        this.theme = localStorage.getItem('theme') || 'light';\n"
                                        "        this.init();\n"
                                        "    }\n"
                                        "    \n"
                                        "    toggleTheme() {\n"
                                        "        this.theme = this.theme === 'light' ? 'dark' : 'light';\n"
                                        "        document.body.dataset.theme = this.theme;\n"
                                        "        localStorage.setItem('theme', this.theme);\n"
                                        "    }\n"
                                        "}"
                                    }
                                }

                                pre {
                                    code class="language-css" {
                                        "/* Ejemplo de CSS */\n"
                                        ".card {\n"
                                        "    background: var(--color-bg-card);\n"
                                        "    border-radius: var(--radius-lg);\n"
                                        "    padding: var(--space-5);\n"
                                        "    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);\n"
                                        "    transition: transform var(--transition-base);\n"
                                        "}\n"
                                        "\n"
                                        ".card:hover {\n"
                                        "    transform: translateY(-2px);\n"
                                        "    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);\n"
                                        "}"
                                    }
                                }
                            }

                            div class="subsection" {
                                h3 { "9.2 Terminal Output" }
                                
                                pre class="terminal" {
                                    samp {
                                        "$ cargo run --release\n"
                                        "   Compiling styles-web v0.1.0\n"
                                        "    Finished release [optimized] target(s) in 2.34s\n"
                                        "     Running `target/release/styles-web`\n"
                                        "Server running on http://127.0.0.1:3000"
                                    }
                                }
                            }
                        }

                        // Interactive Section
                        section class="content-section" {
                            h2 class="section-title" { "10. Elementos Interactivos" }
                            
                            div class="subsection" {
                                h3 { "10.1 Details y Summary" }
                                
                                details {
                                    summary { "Click para expandir información adicional" }
                                    p {
                                        "Este contenido está oculto por defecto y se muestra al hacer click en el summary. "
                                        "Es útil para FAQs, documentación colapsable y contenido opcional."
                                    }
                                }
                                
                                details open {
                                    summary { "Este está abierto por defecto" }
                                    p { "Contenido visible desde el inicio." }
                                }
                            }

                            div class="subsection" {
                                h3 { "10.2 Dialog (Modal)" }
                                
                                button class="btn btn-primary" onclick="document.getElementById('demo-dialog').showModal()" {
                                    "Abrir Modal"
                                }
                                
                                dialog id="demo-dialog" {
                                    h3 { "Dialog Modal" }
                                    p { "Este es un elemento dialog nativo de HTML5." }
                                    form method="dialog" {
                                        button class="btn btn-secondary" { "Cerrar" }
                                    }
                                }
                            }

                            div class="subsection" {
                                h3 { "10.3 Meter y Progress" }
                                
                                label for="disk-usage" { "Uso de disco:" }
                                meter id="disk-usage" value="6" min="0" max="10" { "6 de 10" }
                                
                                br {}
                                
                                label for="task-progress" { "Progreso de tarea:" }
                                progress id="task-progress" value="70" max="100" { "70%" }
                            }
                        }

                        // Accessibility Section
                        section class="content-section" {
                            h2 class="section-title" { "11. Accesibilidad" }
                            
                            div class="subsection" {
                                h3 { "11.1 Navegación por Teclado" }
                                p {
                                    "Todos los elementos interactivos son accesibles mediante teclado. "
                                    "Prueba navegando con " kbd { "Tab" } ", " kbd { "Shift+Tab" } ", "
                                    kbd { "Enter" } " y " kbd { "Espacio" } "."
                                }
                            }

                            div class="subsection" {
                                h3 { "11.2 ARIA Labels y Roles" }
                                
                                nav aria-label="Breadcrumb" {
                                    ol class="breadcrumb" {
                                        li { a href="#" { "Inicio" } }
                                        li { a href="#" { "Productos" } }
                                        li aria-current="page" { "Electric Eclipse" }
                                    }
                                }
                                
                                div role="status" aria-live="polite" aria-atomic="true" {
                                    "Los cambios en este contenido serán anunciados por lectores de pantalla."
                                }
                            }

                            div class="subsection" {
                                h3 { "11.3 Skip Links" }
                                
                                a href="#main-content" class="skip-link" { "Saltar al contenido principal" }
                                a href="#navigation" class="skip-link" { "Saltar a navegación" }
                            }
                        }

                        // Print Styles Section
                        section class="content-section no-print" {
                            h2 class="section-title" { "12. Estilos de Impresión" }
                            
                            div class="subsection" {
                                p {
                                    "Esta sección no se mostrará al imprimir. Los estilos de impresión están "
                                    "optimizados para legibilidad y ahorro de tinta."
                                }
                                
                                div class="print-only" {
                                    p { "Este contenido SOLO aparece al imprimir." }
                                }
                            }
                        }
                    }
                }

                footer {
                    div class="container" {
                        p {
                            "Made by "
                            a href="https://arkaitz.dev" target="_arkaitzdev_website" { "ArkaitzDev" }
                        }
                    }
                }

                script src="/static/js/main.js" defer {}
            }
        }
    }
}

pub fn render_not_found() -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "404 - Página no encontrada | Styles Web" }
                
                // Stylesheets
                link rel="stylesheet" href="/static/css/electric-eclipse/main.css";
                link id="theme-stylesheet" rel="stylesheet" href="/static/css/electric-eclipse/light.css";
                
                // Scripts
                script src="/static/js/theme-init.js" defer {}
            }
            body {
                main {
                    div class="container" {
                        div class="error-section" {
                            h1 { "404" }
                            h2 { "Página no encontrada" }
                            p { "Lo sentimos, la página que buscas no existe." }
                            a href="/" class="btn btn-primary" { "Volver al inicio" }
                        }
                    }
                }
                
                script src="/static/js/main.js" defer {}
            }
        }
    }
}