use crate::domain::entity::session::Session;
use crate::domain::value_object::point::Point;

pub struct CommonEditingSession<StaticContext, InferenceContext> {
    session: Session,
    points: Option<Vec<Point>>,
    static_context: StaticContext,
    inference_context: InferenceContext,
}

pub trait EditingSession {
    type StaticContext;
    type InferenceContext;

    fn session(&self) -> &Session;
    fn points(&self) -> Option<&[Point]>;
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

    fn session(&self) -> &Session {
        &self.session
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
