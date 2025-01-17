// MIT License
//
// Copyright (c) 2024 Saif Ul Islam (Rubix982)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// Structure to represent the whole conversation data
#[derive(Debug, Deserialize, Serialize)]
struct Conversation {
    #[serde(rename = "children")]
    children: Vec<String>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "message")]
    message: Option<Message>,
    #[serde(rename = "parent")]
    parent: Option<String>,
}

// Structure for message metadata and content
#[derive(Debug, Deserialize, Serialize)]
struct Message {
    #[serde(rename = "author")]
    author: Author,
    #[serde(rename = "content")]
    content: Content,
    #[serde(rename = "create_time")]
    create_time: Option<f64>,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "metadata")]
    metadata: Option<Metadata>,
    #[serde(rename = "recipient")]
    recipient: String,
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "weight")]
    weight: i32,
    #[serde(rename = "end_turn")]
    end_turn: Option<bool>,
}

// Structure to represent the message author
#[derive(Debug, Deserialize, Serialize)]
struct Author {
    #[serde(rename = "metadata")]
    metadata: Value,
    #[serde(rename = "role")]
    role: String,
}

// Structure for the content of the message
#[derive(Debug, Deserialize, Serialize)]
struct Content {
    #[serde(rename = "content_type")]
    content_type: String,
    #[serde(rename = "parts")]
    parts: Vec<String>,
}

// Structure to store additional metadata about the message
#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    #[serde(rename = "is_visually_hidden_from_conversation")]
    is_visually_hidden_from_conversation: Option<bool>,
    #[serde(rename = "shared_conversation_id")]
    shared_conversation_id: Option<String>,
    #[serde(rename = "message_source")]
    message_source: Option<Value>,
    #[serde(rename = "message_type")]
    message_type: Option<Value>,
    #[serde(rename = "request_id")]
    request_id: Option<String>,
    #[serde(rename = "serialization_metadata")]
    serialization_metadata: Option<SerializationMetadata>,
    #[serde(rename = "finish_details")]
    finish_details: Option<FinishDetails>,
    #[serde(rename = "parent_id")]
    parent_id: Option<String>,
    #[serde(rename = "citations")]
    citations: Option<Vec<Value>>,
    #[serde(rename = "content_references")]
    content_references: Option<Vec<Value>>,
    #[serde(rename = "model_slug")]
    model_slug: Option<String>,
    #[serde(rename = "timestamp_")]
    timestamp_: Option<String>,
    #[serde(rename = "weight")]
    weight: Option<i32>,
}

// Structure for handling serialization metadata
#[derive(Debug, Deserialize, Serialize)]
struct SerializationMetadata {
    #[serde(rename = "custom_symbol_offsets")]
    custom_symbol_offsets: Vec<Value>,
}

// Structure for finish details in metadata
#[derive(Debug, Deserialize, Serialize)]
struct FinishDetails {
    #[serde(rename = "stop_tokens")]
    stop_tokens: Option<Vec<i32>>,
    #[serde(rename = "type")]
    type_: Option<String>,
}

// The root structure representing the entire conversation list
#[derive(Debug, Deserialize, Serialize)]
struct ConversationList(Vec<Conversation>);
