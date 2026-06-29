use crate::{application::types::point_history::PointHistory, domain::value_object::point::Point};

pub struct CommonEditingSession<StaticContext, InferenceContext> {
    history: PointHistory,
    static_context: StaticContext,
    inference_context: InferenceContext,
}

pub trait EditingSession {
    type StaticContext;
    type InferenceContext;

    fn undo(&mut self) -> bool;
    fn redo(&mut self) -> bool;
    fn points(&self) -> &[Point];
    fn add_point(&mut self, point: Point);
    fn static_context(&self) -> &Self::StaticContext;
    fn inference_context(&self) -> &Self::InferenceContext;

    fn set_inference_context(&mut self, inference_context: Self::InferenceContext);
}

impl<S, I> EditingSession for CommonEditingSession<S, I> {
    type InferenceContext = I;
    type StaticContext = S;

    fn undo(&mut self) -> bool {
        self.history.undo()
    }

    fn redo(&mut self) -> bool {
        self.history.redo()
    }

    fn points(&self) -> &[Point] {
        self.history.current()
    }

    fn add_point(&mut self, point: Point) {
        self.history.add(point);
    }

    fn inference_context(&self) -> &Self::InferenceContext {
        &self.inference_context
    }

    fn static_context(&self) -> &Self::StaticContext {
        &self.static_context
    }

    fn set_inference_context(&mut self, inference_context: Self::InferenceContext) {
        self.inference_context = inference_context
    }
}

impl<S, I> CommonEditingSession<S, I> {
    pub fn new(history: PointHistory, static_context: S, inference_context: I) -> Self {
        Self {
            history: history,
            static_context: static_context,
            inference_context: inference_context,
        }
    }
}
