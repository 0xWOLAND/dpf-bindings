use std::ffi::{c_char, c_int};
use std::ptr;

#[repr(C)]
pub enum DpfStatus {
    Success = 0,
    InvalidArgument = 1,
    Internal = 2,
    OutOfMemory = 3,
}

// Opaque types
#[repr(C)]
pub struct DpfDatabase {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DpfClient {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DpfServer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DpfRequest {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DpfResponse {
    _private: [u8; 0],
}

// Raw C bindings
extern "C" {
    fn DpfCreateDatabase(num_elements: c_int, element_size_bytes: c_int, db: *mut *mut DpfDatabase) -> DpfStatus;
    fn DpfDestroyDatabase(db: *mut DpfDatabase) -> DpfStatus;
    fn DpfDatabaseSetElement(db: *mut DpfDatabase, index: c_int, data: *const c_char, data_size: c_int) -> DpfStatus;
    
    fn DpfCreateClient(client: *mut *mut DpfClient) -> DpfStatus;
    fn DpfDestroyClient(client: *mut DpfClient) -> DpfStatus;
    
    fn DpfCreateServer(db: *mut DpfDatabase, server: *mut *mut DpfServer) -> DpfStatus;
    fn DpfDestroyServer(server: *mut DpfServer) -> DpfStatus;
    
    fn DpfCreateRequest(client: *mut DpfClient, index: c_int, request: *mut *mut DpfRequest) -> DpfStatus;
    fn DpfDestroyRequest(request: *mut DpfRequest) -> DpfStatus;
    
    fn DpfProcessRequest(server: *mut DpfServer, request: *mut DpfRequest, response: *mut *mut DpfResponse) -> DpfStatus;
    fn DpfDestroyResponse(response: *mut DpfResponse) -> DpfStatus;
    
    fn DpfGetResult(client: *mut DpfClient, request: *mut DpfRequest, response: *mut DpfResponse, result: *mut *mut c_char, result_size: *mut c_int) -> DpfStatus;
    fn DpfDestroyResult(result: *mut c_char) -> DpfStatus;
}

// Safe Rust wrappers

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    Internal,
    OutOfMemory,
}

impl From<DpfStatus> for Result<(), Error> {
    fn from(status: DpfStatus) -> Self {
        match status {
            DpfStatus::Success => Ok(()),
            DpfStatus::InvalidArgument => Err(Error::InvalidArgument),
            DpfStatus::Internal => Err(Error::Internal),
            DpfStatus::OutOfMemory => Err(Error::OutOfMemory),
        }
    }
}

fn check_status(status: DpfStatus) -> Result<(), Error> {
    Result::from(status)
}

pub struct Database {
    inner: *mut DpfDatabase,
}

impl Database {
    pub fn new(num_elements: i32, element_size_bytes: i32) -> Result<Self, Error> {
        let mut db = ptr::null_mut();
        unsafe {
            check_status(DpfCreateDatabase(num_elements, element_size_bytes, &mut db))?;
        }
        Ok(Database { inner: db })
    }
    
    pub fn set_element(&mut self, index: i32, data: &[u8]) -> Result<(), Error> {
        unsafe {
            check_status(DpfDatabaseSetElement(
                self.inner,
                index,
                data.as_ptr() as *const c_char,
                data.len() as c_int,
            ))
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            DpfDestroyDatabase(self.inner);
        }
    }
}

pub struct Client {
    inner: *mut DpfClient,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let mut client = ptr::null_mut();
        unsafe {
            check_status(DpfCreateClient(&mut client))?;
        }
        Ok(Client { inner: client })
    }
    
    pub fn create_request(&mut self, index: i32) -> Result<Request, Error> {
        let mut request = ptr::null_mut();
        unsafe {
            check_status(DpfCreateRequest(self.inner, index, &mut request))?;
        }
        Ok(Request { inner: request })
    }
    
    pub fn get_result(&mut self, request: &Request, response: &Response) -> Result<Vec<u8>, Error> {
        let mut result = ptr::null_mut();
        let mut result_size = 0;
        unsafe {
            check_status(DpfGetResult(self.inner, request.inner, response.inner, &mut result, &mut result_size))?;
            
            let data = std::slice::from_raw_parts(result as *const u8, result_size as usize).to_vec();
            DpfDestroyResult(result);
            Ok(data)
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe {
            DpfDestroyClient(self.inner);
        }
    }
}

pub struct Server {
    inner: *mut DpfServer,
}

impl Server {
    pub fn new(db: &mut Database) -> Result<Self, Error> {
        let mut server = ptr::null_mut();
        unsafe {
            check_status(DpfCreateServer(db.inner, &mut server))?;
        }
        Ok(Server { inner: server })
    }
    
    pub fn process_request(&mut self, request: &Request) -> Result<Response, Error> {
        let mut response = ptr::null_mut();
        unsafe {
            check_status(DpfProcessRequest(self.inner, request.inner, &mut response))?;
        }
        Ok(Response { inner: response })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe {
            DpfDestroyServer(self.inner);
        }
    }
}

pub struct Request {
    inner: *mut DpfRequest,
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe {
            DpfDestroyRequest(self.inner);
        }
    }
}

pub struct Response {
    inner: *mut DpfResponse,
}

impl Drop for Response {
    fn drop(&mut self) {
        unsafe {
            DpfDestroyResponse(self.inner);
        }
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pir_flow() -> Result<(), Error> {
        // Create database with 10 elements of 8 bytes each
        let mut db = Database::new(10, 8)?;
        
        // Set some elements
        db.set_element(0, &[1, 2, 3, 4, 5, 6, 7, 8])?;
        db.set_element(1, &[8, 7, 6, 5, 4, 3, 2, 1])?;
        
        // Create client and server
        let mut client = Client::new()?;
        let mut server = Server::new(&mut db)?;
        
        // Create request for index 1
        let request = client.create_request(1)?;
        
        // Process request on server
        let response = server.process_request(&request)?;
        
        // Get result on client
        let result = client.get_result(&request, &response)?;
        
        // Verify result
        assert_eq!(result, vec![8, 7, 6, 5, 4, 3, 2, 1]);
        
        Ok(())
    }
}
