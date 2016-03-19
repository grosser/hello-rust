#[cfg(test)]

mod tests {
    pub use super::CanvasRenderer;
    pub use super::Canvas;

    describe! canvas_renderer {

        before_each {
            let mut canvas = Canvas {
                width: 10,
                height: 10,
                array: vec!['x';10*10],
            };
        }

        it "should fill given char at given coords" {
            {
                let mut renderer: CanvasRenderer = CanvasRenderer::new(&mut canvas);
                renderer.render_point('x', 3,3);
            }
            assert_eq!('x', canvas.array[3*3]);
        }
    }
}

