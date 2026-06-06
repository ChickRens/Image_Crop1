use crate::domain::value_object::point::Point;

pub struct CommonEditingSession<StaticContext, InferenceContext> {
    points: Option<Vec<Point>>,
    static_context: StaticContext,
    inference_context: InferenceContext,
}

pub trait EditingSession {
    type StaticContext;
    type InferenceContext;

    fn points(&self) -> Option<&[Point]>;
    fn update_points(&mut self, points: Vec<Point>);
    fn static_context(&self) -> &Self::StaticContext;
    fn inference_context(&self) -> &Self::InferenceContext;

    fn inference_context_mut(&mut self) -> &mut Self::InferenceContext;
}

impl<S, I> EditingSession for CommonEditingSession<S, I> {
    type InferenceContext = I;
    type StaticContext = S;

    fn points(&self) -> Option<&[Point]> {
        self.points.as_deref()
    }

    fn update_points(&mut self, points: Vec<Point>) {
        self.points = Some(points)
    }

    fn inference_context(&self) -> &Self::InferenceContext {
        &self.inference_context
    }

    fn static_context(&self) -> &Self::StaticContext {
        &self.static_context
    }

    fn inference_context_mut(&mut self) -> &mut Self::InferenceContext {
        &mut self.inference_context
    }
}

impl<S, I> CommonEditingSession<S, I> {
    pub fn new(static_context: S, inference_context: I) -> Self {
        Self {
            points: None,
            static_context: static_context,
            inference_context: inference_context,
        }
    }
}
