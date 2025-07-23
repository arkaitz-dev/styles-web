use crate::{get_person_json_ld, get_website_json_ld};
use maud::{html, Markup, DOCTYPE};

pub fn render_hello_world() -> Markup {
    html! {
        (DOCTYPE)
        html lang="es" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Styles Web - Hola Mundo" }
                meta name="description" content="Una página de prueba de estilos con un simple Hola Mundo";
                meta name="keywords" content="hola mundo, styles, css, html";
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
                        div class="hero-section" {
                            h1 { "¡Hola Mundo!" }
                            p class="lead" { "Esta es una página de prueba para demostrar los estilos del tema Electric Eclipse." }
                            
                            div class="content-section" {
                                h2 { "Características incluidas:" }
                                ul {
                                    li { "Sistema de temas claro/oscuro con cambio dinámico" }
                                    li { "Estilos CSS del tema Electric Eclipse" }
                                    li { "Configuración completa de metadatos y SEO" }
                                    li { "Scripts HTMX incluidos (aunque no necesarios para esta página simple)" }
                                    li { "Estructura de datos JSON-LD para SEO avanzado" }
                                    li { "Headers de seguridad configurados" }
                                    li { "Favicon y iconos de aplicación" }
                                }
                                
                                h2 { "Elementos de prueba:" }
                                p { "Este párrafo demuestra el estilo de texto normal con el sistema de tipos configurado." }
                                
                                blockquote {
                                    "Este es un blockquote de ejemplo para mostrar cómo se ve el texto citado con los estilos aplicados."
                                }
                                
                                div class="button-group" {
                                    button class="btn btn-primary" { "Botón Primario" }
                                    button class="btn btn-secondary" { "Botón Secundario" }
                                }
                                
                                div class="code-example" {
                                    pre {
                                        code {
                                            "function holamundo() {\n"
                                            "    console.log('¡Hola Mundo desde JavaScript!');\n"
                                            "}"
                                        }
                                    }
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