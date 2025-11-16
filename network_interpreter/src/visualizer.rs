// ============================================================================
// VISUALIZADOR - FASE 5
// ============================================================================
// Visualiza la topología de red usando iced GUI framework
// Dibuja máquinas, concentradores, cables y conexiones

use iced::widget::{canvas, column, container, text, Canvas};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};
use iced::widget::canvas::{Frame, Geometry, Path, Stroke};

use crate::interpreter::{Environment, ConexionMaquina};

// ============================================================================
// MENSAJES
// ============================================================================

#[derive(Debug, Clone)]
pub enum Message {
    // Por ahora no necesitamos mensajes interactivos
}

// ============================================================================
// ESTADO DE LA APLICACIÓN
// ============================================================================

#[derive(Clone)]
pub struct NetworkVisualizer {
    env: Environment,
}

impl NetworkVisualizer {
    fn new(env: Environment) -> Self {
        Self { env }
    }

    fn update(&mut self, _message: Message) {
        // Por ahora no manejamos mensajes
    }

    fn view(&self) -> Element<'_, Message> {
        let canvas = Canvas::new(NetworkCanvas {
            env: &self.env,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let content = column![
            text("Topología de Red Ethernet").size(24),
            canvas,
        ]
        .padding(20)
        .spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// ============================================================================
// CANVAS PARA DIBUJAR LA RED
// ============================================================================

struct NetworkCanvas<'a> {
    env: &'a Environment,
}

impl<'a> canvas::Program<Message> for NetworkCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Escala para el dibujo (1 unidad de programa = 1 pixel)
        // Offset para centrar el dibujo
        let offset_x = 50.0;
        let offset_y = 50.0;

        // Dibujar cables coaxiales primero (fondo)
        self.draw_coaxiales(&mut frame, offset_x, offset_y);

        // Dibujar conexiones UTP
        self.draw_conexiones_utp(&mut frame, offset_x, offset_y);

        // Dibujar concentradores
        self.draw_concentradores(&mut frame, offset_x, offset_y);

        // Dibujar máquinas
        self.draw_maquinas(&mut frame, offset_x, offset_y);

        vec![frame.into_geometry()]
    }
}

impl<'a> NetworkCanvas<'a> {
    fn draw_maquinas(&self, frame: &mut Frame, offset_x: f32, offset_y: f32) {
        for (_nombre, maq) in &self.env.maquinas {
            if maq.colocada {
                let x = maq.x as f32 + offset_x;
                let y = maq.y as f32 + offset_y;

                // Dibujar círculo para la máquina
                let circle = Path::circle(Point::new(x, y), 15.0);
                frame.fill(
                    &circle,
                    Color::from_rgb(0.2, 0.6, 1.0), // Azul
                );

                // Borde
                frame.stroke(
                    &circle,
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(Color::from_rgb(0.0, 0.0, 0.0)),
                );

                // Texto con el nombre (simplificado - solo la primera letra por ahora)
                // En iced, el texto en canvas requiere más configuración
                // Por ahora dibujamos un punto central
                let center_dot = Path::circle(Point::new(x, y), 2.0);
                frame.fill(&center_dot, Color::BLACK);
            }
        }
    }

    fn draw_concentradores(&self, frame: &mut Frame, offset_x: f32, offset_y: f32) {
        for (_nombre, conc) in &self.env.concentradores {
            if conc.colocado {
                let x = conc.x as f32 + offset_x;
                let y = conc.y as f32 + offset_y;
                let size = 40.0;

                // Dibujar rectángulo para el concentrador
                let rect = Path::rectangle(
                    Point::new(x - size / 2.0, y - size / 2.0),
                    Size::new(size, size),
                );

                frame.fill(
                    &rect,
                    Color::from_rgb(1.0, 0.7, 0.2), // Naranja
                );

                // Borde
                frame.stroke(
                    &rect,
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(Color::from_rgb(0.0, 0.0, 0.0)),
                );

                // Indicador de salida coaxial
                if conc.tiene_coaxial {
                    let coax_indicator = Path::circle(
                        Point::new(x + size / 2.0, y),
                        5.0,
                    );
                    frame.fill(&coax_indicator, Color::from_rgb(0.8, 0.2, 0.2));
                }
            }
        }
    }

    fn draw_coaxiales(&self, frame: &mut Frame, offset_x: f32, offset_y: f32) {
        for (_nombre, coax) in &self.env.coaxiales {
            if coax.colocado {
                let x1 = coax.x as f32 + offset_x;
                let y1 = coax.y as f32 + offset_y;

                // Calcular punto final según la dirección y longitud
                let (x2, y2) = match coax.direccion.as_str() {
                    "Derecha" | "\"Derecha\"" => (x1 + coax.longitud as f32, y1),
                    "Izquierda" | "\"Izquierda\"" => (x1 - coax.longitud as f32, y1),
                    "Arriba" | "\"Arriba\"" => (x1, y1 - coax.longitud as f32),
                    "Abajo" | "\"Abajo\"" => (x1, y1 + coax.longitud as f32),
                    _ => (x1 + coax.longitud as f32, y1),
                };

                // Dibujar línea gruesa para el cable coaxial
                let line = Path::line(
                    Point::new(x1, y1),
                    Point::new(x2, y2),
                );

                frame.stroke(
                    &line,
                    Stroke::default()
                        .with_width(4.0)
                        .with_color(Color::from_rgb(0.3, 0.3, 0.3)), // Gris oscuro
                );
            }
        }
    }

    fn draw_conexiones_utp(&self, frame: &mut Frame, offset_x: f32, offset_y: f32) {
        for (_nombre, maq) in &self.env.maquinas {
            if let Some(ConexionMaquina::Puerto { concentrador, puerto: _ }) = &maq.conectada_a {
                if maq.colocada {
                    if let Some(conc) = self.env.concentradores.get(concentrador) {
                        if conc.colocado {
                            let x1 = maq.x as f32 + offset_x;
                            let y1 = maq.y as f32 + offset_y;
                            let x2 = conc.x as f32 + offset_x;
                            let y2 = conc.y as f32 + offset_y;

                            // Dibujar línea punteada para cable UTP
                            let line = Path::line(
                                Point::new(x1, y1),
                                Point::new(x2, y2),
                            );

                            frame.stroke(
                                &line,
                                Stroke::default()
                                    .with_width(1.5)
                                    .with_color(Color::from_rgb(0.4, 0.8, 0.4)), // Verde
                            );
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// FUNCIÓN PARA LANZAR EL VISUALIZADOR
// ============================================================================

pub fn run(env: Environment) -> iced::Result {
    use iced::Task;

    // Lanzar aplicación usando la nueva API de iced 0.13
    iced::application(
        "Network Interpreter - Topología de Red",
        |_state: &mut NetworkVisualizer, _msg: Message| {
            // Por ahora no manejamos mensajes, retornar Task::none()
            Task::none()
        },
        NetworkVisualizer::view
    ).run_with(move || {
        // Inicializar estado con el environment
        (NetworkVisualizer::new(env), Task::none())
    })
}
