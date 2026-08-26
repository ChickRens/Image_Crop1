use crate::{
    application::{
        interface::{
            editing_session_repository::repository::EditingSessionRepository,
            image_loader::loader::ImageLoader, image_segmenter::segmenter::ImageSegmenter,
            rendered_image_cache::cache::RenderedImageCache,
        },
        types::{
            editing_session::session::{CommonEditingSession, EditingSession},
            inference_context_history::InferenceContextHistory,
            point_history::PointHistory,
            rendered_image::RenderedImage,
        },
        usecase::{
            config::MAX_HISTORY,
            upload_usecase::{
                error::UploadUseCaseError, upload_input::UploadInput, upload_output::UploadOutput,
            },
        },
    }, domain::{
        entity::{original_image::OriginalImage, session::Session}, repository::{
            original_image_repository::repository::OriginalImageRepository,
            session_repository::repository::SessionRepository,
        }, value_object::{image_id::image_id::ImageId, session_id::session_id::SessionId},
    },
};

pub struct UploadUseCase<SR, IR, LD, IC>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
    LD: ImageLoader,
    IC: RenderedImageCache,
{
    session_repo: SR,
    image_repo: IR,
    loader: LD,
    image_cache: IC,
}

impl<SR, IR, LD, IC> UploadUseCase<SR, IR, LD, IC>
where
    SR: SessionRepository,
    IR: OriginalImageRepository,
    LD: ImageLoader,
    IC: RenderedImageCache,
{
    pub fn new(
        session_repository: SR,
        image_repository: IR,
        image_loader: LD,
        rendered_image_cache: IC,
    ) -> Self {
        Self {
            session_repo: session_repository,
            image_repo: image_repository,
            loader: image_loader,
            image_cache: rendered_image_cache,
        }
    }

    pub fn execute(&self, input: UploadInput) -> Result<UploadOutput, UploadUseCaseError> {
        let input_image = input.into_image_data();
        let loaded_image = self.loader.load(input_image)?;

        let image = loaded_image.into_image();
        let image_id = *image.image_id();

        let session_id = SessionId::new();
        let session = Session::new(session_id, image_id);
        self.session_repo.save(session);
        
        let original_image = OriginalImage::new(image.clone());
        self.image_repo.save(original_image);

        let (image_data, _, size) = image.into_data();

        self.image_cache
            .save(RenderedImage::new(image_data, image_id, size));

        let output = UploadOutput::new(session_id, image_id);

        Ok(output)
    }
}
