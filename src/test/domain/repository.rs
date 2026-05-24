#[cfg(test)]
mod repository_tests {
    use uuid::Uuid;

    use crate::domain::entity::{image::Image, session::Session};
    use crate::domain::repository::image_repository::ImageRepository;
    use crate::domain::repository::session_repository::SessionRepository;
    use crate::domain::value_object::{
        image_data::ImageData, image_id::ImageId, image_kind::ImageKind, image_size::ImageSize,
        session_id::SessionId,
    };
    use crate::infrastructure::repository::image_repository::ImageRepositoryInMemory;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

    #[test]
    fn test_image_repository_save_and_get() {
        let mut repo = ImageRepositoryInMemory::new();
        let image_id = ImageId::from_uuid(Uuid::new_v4());
        let image = Image::new(
            ImageData::new(vec![1, 2, 3]),
            image_id.clone(),
            ImageSize::new(2, 2).unwrap(),
        );

        repo.save(image.clone(), ImageKind::Original);
        let stored = repo.get(&image_id, ImageKind::Original);
        assert!(stored.is_some());
        assert_eq!(stored.unwrap().image_id(), &image_id);
    }

    #[test]
    fn test_session_repository_save_and_get() {
        let mut repo = SessionRepositoryInMemory::new();
        let session_id = SessionId::from_uuid(Uuid::new_v4());
        let image_id = ImageId::from_uuid(Uuid::new_v4());
        let session = Session::new(session_id.clone(), image_id.clone());

        repo.save(session.clone());
        let stored = repo.get(&session_id);

        assert!(stored.is_some());
        assert_eq!(stored.unwrap().session_id(), &session_id);
    }
}
