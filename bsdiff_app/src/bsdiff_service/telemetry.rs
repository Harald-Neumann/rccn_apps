pub struct CreateResponse {
    pub length: u32,
    pub data: Vec<u8>,
}

impl From<CreateResponse> for Vec<u8> {
    fn from(response: CreateResponse) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&response.length.to_be_bytes());
        data.extend_from_slice(&response.data);
        data
    }
}