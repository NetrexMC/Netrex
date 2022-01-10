use std::collections::HashSet;

use binary_utils::*;
use mcpe_protocol::interfaces::ByteArray;
use mcpe_protocol::interfaces::VarString;

// todo
pub struct Skin {}

pub struct SkinDataJwt {
    /// The Skins UUID
    pub skin_id: VarString,
    /// PlayFabID, if available, is a market place company name.
    pub playfab_id: VarString,
    pub resource_patch: ByteArray<u32>,
    pub image: SkinImage,
    pub animations: HashSet<SkinAnimation>,
}

#[derive(BinaryStream, Debug)]
pub struct SkinImage {
    pub width: u32,
    pub height: u32,
    pub data: ByteArray<u32>,
}

impl SkinImage {
    pub fn new(width: u32, height: u32, data: ByteArray<u32>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
}

#[derive(BinaryStream, Debug)]
pub struct SkinAnimation {
    pub animation_type: u32,
    pub frame_count: f32,
    pub expression: u32,
}
