#[cfg(test)]
mod image_meta_repository_in_memory_test {
    use crate::domain::entity::session::Session;
    use crate::domain::repository::session_repository::SessionRepository;
    use crate::domain::value_object::image_id::ImageId;
    use crate::domain::value_object::session_id::SessionId;
    use crate::infrastructure::repository::session_repository::SessionRepositoryInMemory;

    #[test]
    fn test_normal_get() {
        let mut repo = SessionRepositoryInMemory::new();

        let session_id1 = SessionId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let image_id1 = ImageId::from_str("aa223311-2634-49d2-aa9f-bc59db69435d").unwrap();

        let session1 = Session::new(session_id1, image_id1);
        repo.save(session1.clone());

        let got1 = repo.get(&session_id1).unwrap();

        assert_eq!(got1, session1);
    }

    #[test]
    fn test_unknown_image_get() {
        let mut repo = SessionRepositoryInMemory::new();

        let image_id1 = ImageId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();

        let session_id1 = SessionId::from_str("22222222-2222-2222-2222-222222222222").unwrap();
        let session_id2 = SessionId::from_str("33333333-3333-3333-3333-333333333333").unwrap();

        let session1 = Session::new(session_id1, image_id1);
        repo.save(session1);

        let got = repo.get(&session_id2);

        assert_eq!(got, None);
    }

    #[test]
    fn test_overwrite_save_existing_image() {
        let mut repo = SessionRepositoryInMemory::new();

        let session_id = SessionId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let old_image_id = ImageId::from_str("22222222-2222-2222-2222-222222222222").unwrap();
        let new_image_id = ImageId::from_str("11111111-2222-2222-2222-222222222222").unwrap();

        let old_session = Session::new(session_id, old_image_id);
        let new_session = Session::new(session_id, new_image_id);

        repo.save(old_session.clone());
        repo.save(new_session.clone());

        let got = repo.get(&session_id);

        assert_eq!(got, Some(new_session));
    }

    #[test]
    fn test_multi_image() {
        let mut repo = SessionRepositoryInMemory::new();

        let session_id1 = SessionId::from_str("65921fe2-2634-49d2-aa9f-bc59db69435d").unwrap();
        let session_id2 = SessionId::from_str("12345678-9abc-def0-1234-56789abcdef0").unwrap();
        let session_id3 = SessionId::from_str("00000000-0000-0000-0000-000000000000").unwrap();

        let image_id1 = ImageId::from_str("00000000-0000-0000-1111-000000000000").unwrap();
        let image_id2 = ImageId::from_str("00000000-0000-0000-2222-000000000000").unwrap();
        let image_id3 = ImageId::from_str("00000000-0000-0000-3333-000000000000").unwrap();

        let session1 = Session::new(session_id1, image_id1);
        let session2 = Session::new(session_id2, image_id2);
        let session3 = Session::new(session_id3, image_id3);

        repo.save(session1.clone());
        repo.save(session2.clone());
        repo.save(session3.clone());

        let got1 = repo.get(&session_id1);
        let got2 = repo.get(&session_id2);
        let got3 = repo.get(&session_id3);

        assert_eq!(got1, Some(session1));
        assert_eq!(got2, Some(session2));
        assert_eq!(got3, Some(session3));
    }
}
