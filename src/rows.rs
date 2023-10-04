use std::fmt;

use crate::Result;

const ID_SIZE: u32 = 4;
const USERNAME_SIZE: u32 = 32;
const EMAIL_SIZE: u32 = 255;
const ID_OFFSET: u32 = 0;
const USERNAME_OFFSET: u32 = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: u32 = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: u32 = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {

    pub fn new(id: u32, username: &str, email: &str) -> Result<Self> {
        
        if username.len() > USERNAME_SIZE as usize {
            return Err("username is longer than 32".into())
        }
        
        if email.len() > EMAIL_SIZE as usize {
            return Err("email is longer than 255".into())
        }

        Ok( Self { id, username: username.to_string(), email: email.to_string() } )
    }

    pub fn serialize_row(&self, destination: &mut [u8]) -> Result<()> {

        if destination.len() < ROW_SIZE as usize {
            return Err("destination buffer is too small".into())
        }

        destination[ID_OFFSET as usize..ID_SIZE as usize].copy_from_slice(&self.id.to_be_bytes());
        
        let username = &mut destination[USERNAME_OFFSET as usize..(USERNAME_OFFSET + USERNAME_SIZE) as usize];
        username[0..self.username.len()].copy_from_slice(self.username.as_bytes());
        
        let email = &mut destination[EMAIL_OFFSET as usize..(EMAIL_OFFSET + EMAIL_SIZE) as usize];
        email[0..self.email.len()].copy_from_slice(self.email.as_bytes());
    
        Ok(())
    }

    pub fn deserialize_row(&mut self, source: &[u8]) -> Result<Self> {

        if source.len() < ROW_SIZE as usize {
            return Err("cannot deserialize row".into())
        }

        let id = u32::from_be_bytes(source[ID_OFFSET as usize..ID_OFFSET as usize + ID_SIZE as usize].try_into().unwrap());
        let username = String::from_utf8_lossy(&source[USERNAME_OFFSET as usize..(USERNAME_OFFSET + USERNAME_SIZE) as usize]).trim_end_matches(char::from(0)).to_string();
        let email = String::from_utf8_lossy(&source[EMAIL_OFFSET as usize..(EMAIL_OFFSET + EMAIL_SIZE) as usize]).trim_end_matches(char::from(0)).to_string();


        Ok( Self { id, username, email } )
    }

    pub fn set_username(&mut self, value: String) -> Result<()> {
        if value.len() > USERNAME_SIZE as usize {
            return Err("username is longer than 32".into())
        }

        self.username = value;

        Ok(())
    }

    pub fn set_email(&mut self, value: String) -> Result<()> {
        if value.len() >  EMAIL_SIZE as usize {
            return Err("email is longer than 255".into())
        }

        self.email = value;

        Ok(())
    }
}

impl fmt::Display for Row {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Row ({}, {}, {})", self.id,  self.username, self.email)
    }
}