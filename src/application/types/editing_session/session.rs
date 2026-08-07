use crate::{application::types::{editing_session::error::EditingSessionError, inference_context_history::InferenceContextHistory, point_history::PointHistory}, domain::value_object::point::Point};

pub struct CommonEditingSession<StaticContext, InferenceContext> {
    point_history: PointHistory,
    static_context: StaticContext,
    inference_context_history: InferenceContextHistory<InferenceContext>
}

pub trait EditingSession {
    type StaticContext;
    type InferenceContext;

    fn undo(&mut self) -> Result<(), EditingSessionError>;
    fn redo(&mut self) -> Result<(), EditingSessionError>;
    fn points(&self) -> &[Point];
    fn add_point(&mut self, point: Point);
    fn static_context(&self) -> &Self::StaticContext;
    fn inference_context(&self) -> &Self::InferenceContext;

    fn update_inference_context(&mut self, inference_context: Self::InferenceContext);
}

impl<S, I> EditingSession for CommonEditingSession<S, I> {
    type InferenceContext = I;
    type StaticContext = S;

    fn undo(&mut self) -> Result<(), EditingSessionError> {
        if !(self.inference_context_history.can_undo() && self.point_history.can_undo()) {
            return Err(EditingSessionError::UndoFailed);
        }

        self.point_history.undo();
        self.inference_context_history.undo();
        Ok(())
    }

    fn redo(&mut self) -> Result<(), EditingSessionError> {
        if !(self.inference_context_history.can_redo() && self.point_history.can_redo()) {
            return Err(EditingSessionError::RedoFailed);
        }
        self.point_history.redo();
        self.inference_context_history.redo();

        Ok(())
    }

    fn points(&self) -> &[Point] {
        self.point_history.current()
    }

    fn add_point(&mut self, point: Point) {
        self.point_history.add(point);
    }

    fn inference_context(&self) -> &Self::InferenceContext {
        &self.inference_context_history.current()
    }

    fn static_context(&self) -> &Self::StaticContext {
        &self.static_context
    }

    fn update_inference_context(&mut self, inference_context: Self::InferenceContext) {
        self.inference_context_history.add(inference_context);
    }
}

impl<S, I> CommonEditingSession<S, I> {
    pub fn new(history: PointHistory, static_context: S, inference_context_history: InferenceContextHistory<I>) -> Self {
        Self {
            point_history: history,
            static_context: static_context,
            inference_context_history: inference_context_history
        }
    }
}
