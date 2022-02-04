use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

use chrono::Utc;

use crate::models::Listen;
use crate::service::Service;
use crate::storage::Storage;

use self::{mock_spotify::MockSpotify, mock_storage::MockStorage};

mod mock_spotify;
mod mock_storage;
mod parser;

// #[tokio::test]
// async fn test_history_update() {
//     let st = Box::new(MockStorage::new());
//     let sp = MockSpotify::new();
//     let s = Service::new(sp, st);

//     s.history.update().await.unwrap();

//     let listens = st.listens.lock().await.borrow();

//     assert_eq!(listens.len(), 2);

//     assert_eq!(
//         listens[0],
//         Listen {
//             song_id: "song_id_1".to_string(),
//             time: Utc::now()
//         }
//     );
// }
