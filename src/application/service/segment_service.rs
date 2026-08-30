use crate::{application::{interface::{editing_session_repository::repository::EditingSessionRepository, image_segmenter::segmenter::ImageSegmenter, segmenter_input_image_storage::storage::SegmenterInputImageStorage}, service::error::SegmentServiceError, types::editing_session::session::EditingSession}, domain::{entity::image::Image, repository::original_image_repository::repository::OriginalImageRepository, value_object::{image_id::image_id::ImageId, point::Point, session_id::session_id::SessionId}}};

pub struct SegmentService<IS, IR, ESR, SS>
where 
    IS: ImageSegmenter,
    IR: OriginalImageRepository,
    ESR: EditingSessionRepository,
    SS: SegmenterInputImageStorage,
{
    segmenter: IS,
    image_repo: IR,
    session_repo: ESR,
    input_storage: SS,
}

impl<IS, IR, ESR, SS> SegmentService<IS, IR, ESR, SS>
where 
    IS: ImageSegmenter,
    IR: OriginalImageRepository,
    ESR: EditingSessionRepository<
        StaticContext = IS::StaticContext,
        InferenceContext = IS::InferenceContext>,
    SS: SegmenterInputImageStorage,
{
    pub fn new(image_segmenter: IS, image_repository: IR, editing_session_repository: ESR, segmenter_input_storage: SS) -> Self {
        Self { segmenter: image_segmenter, image_repo: image_repository, session_repo: editing_session_repository, input_storage: segmenter_input_storage }
    }

    pub fn segment(&self, session_id: SessionId, image_id: ImageId, point: Point) -> Result<Image, SegmentServiceError> {
        let original_image = self.image_repo.get(&image_id)?.into_image();
        let original_size = original_image.image_size();
        
        let input_image = self.input_storage.get(image_id)?;

        let mut session = self.session_repo.get(&session_id)?;
        let static_context = session.static_context();
        let inference_context = session.inference_context();

        let input_points = session.points_with(point.clone());
        let (new_context, segmented_image) = self.segmenter.segment(&input_image, original_size, static_context, inference_context, &input_points)?;
        session.apply_edit(point, new_context);

        self.session_repo.save(&session_id, session);
        let (segmented_image_data, size) = segmented_image.into_image_and_size();
        let segmented_image_id = ImageId::new();

        Ok(Image::new(segmented_image_data, segmented_image_id, size))
    }
}