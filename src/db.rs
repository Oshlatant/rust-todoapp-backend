use futures::StreamExt;
use mongodb::{bson::Document, Cursor};

pub async fn to_vector(cursor: &mut Cursor<Document>) -> Vec<Document> {
    let mut document_list = Vec::new();

    while let Some(document) = cursor.next().await {
        document_list.push(document.unwrap());
    }

    document_list
}
