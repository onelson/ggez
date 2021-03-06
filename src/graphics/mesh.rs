use ::*;
use graphics::*;
use lyon::tessellation as t;


/// A builder for creating `Mesh`es.
///
/// This allows you to easily make one `Mesh` containing
/// many different complex pieces of geometry.
#[derive(Debug, Clone)]
pub struct MeshBuilder {
    buffer: t::geometry_builder::VertexBuffers<Vertex>,
}

impl MeshBuilder {
    /// Create a new MeshBuilder.
    pub fn new() -> Self {
        MeshBuilder { buffer: t::VertexBuffers::new() }
    }

    /// Create a new mesh for a line of one or more connected segments.
    pub fn line(&mut self, points: &[Point2], width: f32) -> &mut Self {
        self.polyline(DrawMode::Line(width), points)
    }

    /// Create a new mesh for a circle.
    pub fn circle(&mut self,
                  mode: DrawMode,
                  point: Point2,
                  radius: f32,
                  tolerance: f32)
                  -> &mut Self {
        {
            let buffers = &mut self.buffer;
            match mode {
                DrawMode::Fill => {
                    // These builders have to be in separate match arms 'cause they're actually
                    // different types; one is GeometryBuilder<StrokeVertex> and the other is
                    // GeometryBuilder<FillVertex>
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    t::basic_shapes::fill_circle(t::math::point(point.x, point.y),
                                                 radius,
                                                 tolerance,
                                                 builder);
                }
                DrawMode::Line(line_width) => {
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    let options = t::StrokeOptions::default()
                        .with_line_width(line_width)
                        .with_tolerance(tolerance);
                    t::basic_shapes::stroke_circle(t::math::point(point.x, point.y),
                                                   radius,
                                                   &options,
                                                   builder);
                }
            };
        }
        self

    }

    /// Create a new mesh for an ellipse.
    pub fn ellipse(&mut self,
                   mode: DrawMode,
                   point: Point2,
                   radius1: f32,
                   radius2: f32,
                   tolerance: f32)
                   -> &mut Self {
        {
            let buffers = &mut self.buffer;
            use euclid::Length;
            match mode {
                DrawMode::Fill => {
                    // These builders have to be in separate match arms 'cause they're actually
                    // different types; one is GeometryBuilder<StrokeVertex> and the other is
                    // GeometryBuilder<FillVertex>
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    t::basic_shapes::fill_ellipse(t::math::point(point.x, point.y),
                                                  t::math::vec2(radius1, radius2),
                                                  Length::new(0.0),
                                                  tolerance,
                                                  builder);
                }
                DrawMode::Line(line_width) => {
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    let options = t::StrokeOptions::default()
                        .with_line_width(line_width)
                        .with_tolerance(tolerance);
                    t::basic_shapes::stroke_ellipse(t::math::point(point.x, point.y),
                                                    t::math::vec2(radius1, radius2),
                                                    Length::new(0.0),
                                                    &options,
                                                    builder);
                }
            };
        }
        self
    }

    /// Create a new mesh for a series of connected lines.
    pub fn polyline(&mut self, mode: DrawMode, points: &[Point2]) -> &mut Self {
        {
            let buffers = &mut self.buffer;
            let points = points
                .into_iter()
                .map(|ggezpoint| t::math::point(ggezpoint.x, ggezpoint.y));
            match mode {
                DrawMode::Fill => {
                    // These builders have to be in separate match arms 'cause they're actually
                    // different types; one is GeometryBuilder<StrokeVertex> and the other is
                    // GeometryBuilder<FillVertex>
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    let tessellator = &mut t::FillTessellator::new();
                    let options = t::FillOptions::default();
                    t::basic_shapes::fill_polyline(points, tessellator, &options, builder).unwrap();
                }
                DrawMode::Line(width) => {
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    let options = t::StrokeOptions::default().with_line_width(width);
                    t::basic_shapes::stroke_polyline(points, false, &options, builder);
                }
            };
        }
        self
    }

    /// Create a new mesh for a closed polygon
    pub fn polygon(&mut self, mode: DrawMode, points: &[Point2]) -> &mut Self {
        {
            let buffers = &mut self.buffer;
            let points = points
                .into_iter()
                .map(|ggezpoint| t::math::point(ggezpoint.x, ggezpoint.y));
            match mode {
                DrawMode::Fill => {
                    // These builders have to be in separate match arms 'cause they're actually
                    // different types; one is GeometryBuilder<StrokeVertex> and the other is
                    // GeometryBuilder<FillVertex>
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    let tessellator = &mut t::FillTessellator::new();
                    let options = t::FillOptions::default();
                    t::basic_shapes::fill_polyline(points, tessellator, &options, builder).unwrap();
                }
                DrawMode::Line(width) => {
                    let builder = &mut t::BuffersBuilder::new(buffers, VertexBuilder);
                    let options = t::StrokeOptions::default().with_line_width(width);
                    t::basic_shapes::stroke_polyline(points, true, &options, builder);
                }
            };
        }
        self
    }

    /// Create a new `Mesh` from a raw list of triangles.
    ///
    /// Currently does not support UV's or indices.
    pub fn triangles(&mut self, triangles: &[Point2]) -> &mut Self {
        {
            assert_eq!(triangles.len() % 3, 0);
            let tris = triangles
                .iter()
                .cloned()
                .map(|p| {
                    // Gotta turn ggez Point2's into lyon FillVertex's
                        let np = lyon::math::Point2D::new(p.x, p.y);
                        let nv = lyon::math::Vector2D::new(p.x, p.y);
                        t::FillVertex {
                            position: np,
                            normal: nv,
                        }
                    })
                    // BUGGO: TODO: Remove the collect, iterate more nicely.
                    // (Probably means collecting into chunks first, THEN 
                    // converting point types, since we can't chunk an iterator,
                    // only a slice.)
                .collect::<Vec<_>>();
            let tris = tris.chunks(3);
            let builder: &mut t::BuffersBuilder<_, _, _> =
                &mut t::BuffersBuilder::new(&mut self.buffer, VertexBuilder);
            use lyon::tessellation::GeometryBuilder;
            builder.begin_geometry();
            for tri in tris {
                // Ideally this assert makes bounds-checks only happen once.
                assert!(tri.len() == 3);
                let fst = tri[0];
                let snd = tri[1];
                let thd = tri[2];
                let i1 = builder.add_vertex(fst);
                let i2 = builder.add_vertex(snd);
                let i3 = builder.add_vertex(thd);
                builder.add_triangle(i1, i2, i3);

            }
            builder.end_geometry();
        }
        self
    }

    /// Takes the accumulated geometry and load it into GPU memory,
    /// creating a single `Mesh`.
    pub fn build(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let (vbuf, slice) =
            ctx.gfx_context
                .factory
                .create_vertex_buffer_with_slice(&self.buffer.vertices[..],
                                                 &self.buffer.indices[..]);

        Ok(Mesh {
               buffer: vbuf,
               slice: slice,
               blend_mode: None,
           })
    }
}


struct VertexBuilder;

impl t::VertexConstructor<t::FillVertex, Vertex> for VertexBuilder {
    fn new_vertex(&mut self, vertex: t::FillVertex) -> Vertex {
        Vertex {
            pos: [vertex.position.x, vertex.position.y],
            uv: [0.0, 0.0],
        }
    }
}

impl t::VertexConstructor<t::StrokeVertex, Vertex> for VertexBuilder {
    fn new_vertex(&mut self, vertex: t::StrokeVertex) -> Vertex {
        Vertex {
            pos: [vertex.position.x, vertex.position.y],
            uv: [0.0, 0.0],
        }
    }
}


/// 2D polygon mesh.
///
/// All of its methods are just shortcuts for doing the same operations via a `MeshBuilder`.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    buffer: gfx::handle::Buffer<gfx_device_gl::Resources, Vertex>,
    slice: gfx::Slice<gfx_device_gl::Resources>,
    blend_mode: Option<BlendMode>,
}


impl Mesh {
    /// Create a new mesh for a line of one or more connected segments.
    pub fn new_line(ctx: &mut Context, points: &[Point2], width: f32) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        mb.polyline(DrawMode::Line(width), points);
        mb.build(ctx)
    }

    /// Create a new mesh for a circle.
    pub fn new_circle(ctx: &mut Context,
                      mode: DrawMode,
                      point: Point2,
                      radius: f32,
                      tolerance: f32)
                      -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        mb.circle(mode, point, radius, tolerance);
        mb.build(ctx)
    }

    /// Create a new mesh for an ellipse.
    pub fn new_ellipse(ctx: &mut Context,
                       mode: DrawMode,
                       point: Point2,
                       radius1: f32,
                       radius2: f32,
                       tolerance: f32)
                       -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        mb.ellipse(mode, point, radius1, radius2, tolerance);
        mb.build(ctx)
    }

    /// Create a new mesh for series of connected lines
    pub fn new_polyline(ctx: &mut Context, mode: DrawMode, points: &[Point2]) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        mb.polyline(mode, points);
        mb.build(ctx)
    }


    /// Create a new mesh for closed polygon
    pub fn new_polygon(ctx: &mut Context, mode: DrawMode, points: &[Point2]) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        mb.polygon(mode, points);
        mb.build(ctx)
    }

    /// Create a new `Mesh` from a raw list of triangles.
    pub fn from_triangles(ctx: &mut Context, triangles: &[Point2]) -> GameResult<Mesh> {
        let mut mb = MeshBuilder::new();
        mb.triangles(triangles);
        mb.build(ctx)
    }
}

impl Drawable for Mesh {
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let gfx = &mut ctx.gfx_context;
        gfx.update_instance_properties(param)?;

        gfx.data.vbuf = self.buffer.clone();
        gfx.data.tex.0 = gfx.white_image.texture.clone();

        gfx.draw(Some(&self.slice))?;

        Ok(())
    }
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.blend_mode = mode;
    }
    fn get_blend_mode(&self) -> Option<BlendMode> {
        self.blend_mode
    }
}
