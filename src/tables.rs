
const ID_SIZE: u32 = 4;
const USERNAME_SIZE: u32 = 32;
const EMAIL_SIZE: u32 = 255;
const ID_OFFSET: u32 = 0;
const USERNAME_OFFSET: u32 = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: u32 = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: u32 = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub struct Row {
    pub id: u32,
    pub username: [u8; 32],
    pub email: [u8; 255],
}

impl Row {

    pub fn serialize_row(&self, destination: &mut [u8]) {

        destination[ID_OFFSET as usize..ID_SIZE as usize].copy_from_slice(&self.id.to_be_bytes());
        destination[USERNAME_OFFSET as usize..USERNAME_SIZE as usize].copy_from_slice(&self.username);
        destination[EMAIL_OFFSET as usize..EMAIL_SIZE as usize].copy_from_slice(&self.email);
    }

    pub fn deserialize_row(&mut self, source: &[u8]) {

        let _ = &mut self.id.to_be_bytes().copy_from_slice(&source[ID_OFFSET as usize..ID_SIZE as usize]);
        self.username.copy_from_slice(&source[USERNAME_OFFSET as usize..USERNAME_SIZE as usize]);
        self.email.copy_from_slice(&source[EMAIL_OFFSET as usize..EMAIL_SIZE as usize])
    }
}